use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use indexed_db_futures::prelude::*;
use indexed_db_futures::web_sys::DomException;
use thiserror::Error;
use wasm_bindgen::JsValue;

use crate::state::Document;

const DB_NAME: &str = "nabu_db";
const DB_VERSION: u32 = 1;
const DOCUMENTS_STORE: &str = "documents";

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IndexedDB error: {0}")]
    IndexedDb(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Encryption error: {0}")]
    Encryption(String),
}

impl From<JsValue> for StorageError {
    fn from(value: JsValue) -> Self {
        StorageError::IndexedDb(format!("{:?}", value))
    }
}

impl From<DomException> for StorageError {
    fn from(value: DomException) -> Self {
        StorageError::IndexedDb(format!("{:?}", value))
    }
}

pub struct Storage {
    key: [u8; 32],
}

impl Storage {
    pub fn new() -> Self {
        let key = Self::get_or_create_key();
        Self { key }
    }

    fn generate_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        getrandom::getrandom(&mut key)
            .expect("browser cryptographic random source is required for document encryption");
        key
    }

    fn get_or_create_key() -> [u8; 32] {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(key_b64)) = storage.get_item("nabu_key") {
                    if let Ok(key_bytes) = BASE64.decode(&key_b64) {
                        if key_bytes.len() == 32 {
                            let mut key = [0u8; 32];
                            key.copy_from_slice(&key_bytes);
                            return key;
                        }
                    }
                }

                let key = Self::generate_key();
                let key_b64 = BASE64.encode(key);
                let _ = storage.set_item("nabu_key", &key_b64);
                return key;
            }
        }

        Self::generate_key()
    }

    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, StorageError> {
        let key = Key::<Aes256Gcm>::from_slice(&self.key);
        let cipher = Aes256Gcm::new(key);

        let mut nonce_bytes = [0u8; 12];
        getrandom::getrandom(&mut nonce_bytes)
            .map_err(|e| StorageError::Encryption(e.to_string()))?;
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| StorageError::Encryption(e.to_string()))?;

        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);
        Ok(result)
    }

    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, StorageError> {
        if data.len() < 12 {
            return Err(StorageError::Encryption("Data too short".into()));
        }

        let key = Key::<Aes256Gcm>::from_slice(&self.key);
        let cipher = Aes256Gcm::new(key);

        let nonce = Nonce::from_slice(&data[..12]);
        let ciphertext = &data[12..];

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| StorageError::Encryption(e.to_string()))
    }

    async fn open_db(&self) -> Result<IdbDatabase, StorageError> {
        let mut db_req = IdbDatabase::open_u32(DB_NAME, DB_VERSION)?;

        db_req.set_on_upgrade_needed(Some(|evt: &IdbVersionChangeEvent| -> Result<(), JsValue> {
            let db = evt.db();

            if !db.object_store_names().any(|n| n == DOCUMENTS_STORE) {
                db.create_object_store(DOCUMENTS_STORE)?;
            }
            Ok(())
        }));

        db_req
            .await
            .map_err(|e| StorageError::IndexedDb(format!("{:?}", e)))
    }

    pub async fn save_document(&self, doc: &Document) -> Result<(), StorageError> {
        let db = self.open_db().await?;
        let tx = db.transaction_on_one_with_mode(DOCUMENTS_STORE, IdbTransactionMode::Readwrite)?;
        let store = tx.object_store(DOCUMENTS_STORE)?;

        let json =
            serde_json::to_vec(doc).map_err(|e| StorageError::Serialization(e.to_string()))?;
        let encrypted = self.encrypt(&json)?;
        let b64 = BASE64.encode(&encrypted);

        store.put_key_val(&JsValue::from_str(&doc.id), &JsValue::from_str(&b64))?;
        tx.await.into_result()?;

        Ok(())
    }

    pub async fn get_document(&self, id: &str) -> Result<Option<Document>, StorageError> {
        let db = self.open_db().await?;
        let tx = db.transaction_on_one(DOCUMENTS_STORE)?;
        let store = tx.object_store(DOCUMENTS_STORE)?;

        let result = store.get(&JsValue::from_str(id))?.await?;

        if let Some(js_val) = result {
            if let Some(b64) = js_val.as_string() {
                let encrypted = BASE64
                    .decode(&b64)
                    .map_err(|e| StorageError::Serialization(e.to_string()))?;
                let decrypted = self.decrypt(&encrypted)?;
                let doc: Document = serde_json::from_slice(&decrypted)
                    .map_err(|e| StorageError::Serialization(e.to_string()))?;
                return Ok(Some(doc));
            }
        }

        Ok(None)
    }

    pub async fn get_all_documents(&self) -> Result<Vec<Document>, StorageError> {
        let db = self.open_db().await?;
        let tx = db.transaction_on_one(DOCUMENTS_STORE)?;
        let store = tx.object_store(DOCUMENTS_STORE)?;

        let cursor = store.open_cursor()?.await?;
        let mut documents = Vec::new();

        if let Some(cursor) = cursor {
            loop {
                if let Some(b64) = cursor.value().as_string() {
                    if let Ok(encrypted) = BASE64.decode(&b64) {
                        if let Ok(decrypted) = self.decrypt(&encrypted) {
                            if let Ok(doc) = serde_json::from_slice::<Document>(&decrypted) {
                                documents.push(doc);
                            }
                        }
                    }
                }

                if !cursor.continue_cursor()?.await? {
                    break;
                }
            }
        }

        Ok(documents)
    }

    pub async fn delete_document(&self, id: &str) -> Result<(), StorageError> {
        let db = self.open_db().await?;
        let tx = db.transaction_on_one_with_mode(DOCUMENTS_STORE, IdbTransactionMode::Readwrite)?;
        let store = tx.object_store(DOCUMENTS_STORE)?;

        store.delete(&JsValue::from_str(id))?;
        tx.await.into_result()?;

        Ok(())
    }
}
