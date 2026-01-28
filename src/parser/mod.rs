use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, FileReader};
use js_sys::{Uint8Array, Function, Promise};
use thiserror::Error;
use pulldown_cmark::{Parser, Event, Tag, TagEnd};

use crate::state::Document;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = extractPdfText)]
    fn extract_pdf_text(array_buffer: &JsValue) -> Promise;

    #[wasm_bindgen(js_namespace = window, js_name = extractDocxText)]
    fn extract_docx_text(array_buffer: &JsValue) -> Promise;
}

const MAX_FILE_SIZE: u64 = 50 * 1024 * 1024; // 50MB

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("File too large (max 50MB)")]
    FileTooLarge,
    #[error("Unsupported file type: {0}")]
    UnsupportedType(String),
    #[error("Failed to read file: {0}")]
    ReadError(String),
    #[error("Failed to parse file: {0}")]
    ParseError(String),
    #[error("Invalid file format")]
    InvalidFormat,
}

pub async fn parse_file(file: File) -> Result<Document, ParseError> {
    let size = file.size() as u64;
    if size > MAX_FILE_SIZE {
        return Err(ParseError::FileTooLarge);
    }

    let name = file.name();
    let file_type = get_file_type(&name)?;

    validate_magic_bytes(&file, &file_type).await?;

    let content = match file_type.as_str() {
        "TXT" => parse_txt(file).await?,
        "MD" => parse_markdown(file).await?,
        "PDF" => parse_pdf(file).await?,
        "DOCX" => parse_docx(file).await?,
        _ => return Err(ParseError::UnsupportedType(file_type)),
    };

    let sanitized = sanitize_content(&content);

    let title = extract_title(&name);
    let mut doc = Document::new(title, sanitized, file_type);

    Ok(doc)
}

fn get_file_type(name: &str) -> Result<String, ParseError> {
    let ext = name.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "txt" => Ok("TXT".to_string()),
        "md" | "markdown" => Ok("MD".to_string()),
        "pdf" => Ok("PDF".to_string()),
        "docx" => Ok("DOCX".to_string()),
        _ => Err(ParseError::UnsupportedType(ext)),
    }
}

async fn validate_magic_bytes(file: &File, file_type: &str) -> Result<(), ParseError> {
    let slice = file.slice_with_f64_and_f64(0.0, 8.0)
        .map_err(|_| ParseError::ReadError("Failed to slice file".into()))?;

    let array_buffer = JsFuture::from(slice.array_buffer())
        .await
        .map_err(|_| ParseError::ReadError("Failed to read slice".into()))?;

    let bytes = Uint8Array::new(&array_buffer).to_vec();

    match file_type {
        "PDF" => {
            if bytes.len() >= 4 && &bytes[0..4] != b"%PDF" {
                return Err(ParseError::InvalidFormat);
            }
        }
        "DOCX" => {
            if bytes.len() >= 4 && &bytes[0..4] != &[0x50, 0x4B, 0x03, 0x04] {
                return Err(ParseError::InvalidFormat);
            }
        }
        _ => {}
    }

    Ok(())
}

async fn read_file_as_text(file: File) -> Result<String, ParseError> {
    let reader = FileReader::new()
        .map_err(|_| ParseError::ReadError("Failed to create FileReader".into()))?;

    let (tx, rx) = futures_channel::oneshot::channel::<Result<String, ParseError>>();
    let tx = std::rc::Rc::new(std::cell::RefCell::new(Some(tx)));

    let onload = {
        let reader_clone = reader.clone();
        let tx = tx.clone();
        wasm_bindgen::closure::Closure::once(Box::new(move || {
            let result = reader_clone.result();
            if let Some(tx) = tx.borrow_mut().take() {
                match result {
                    Ok(val) => {
                        if let Some(text) = val.as_string() {
                            let _ = tx.send(Ok(text));
                        } else {
                            let _ = tx.send(Err(ParseError::ReadError("Not a string".into())));
                        }
                    }
                    Err(_) => {
                        let _ = tx.send(Err(ParseError::ReadError("Read failed".into())));
                    }
                }
            }
        }) as Box<dyn FnOnce()>)
    };

    let onerror = {
        let tx = tx.clone();
        wasm_bindgen::closure::Closure::once(Box::new(move || {
            if let Some(tx) = tx.borrow_mut().take() {
                let _ = tx.send(Err(ParseError::ReadError("FileReader error".into())));
            }
        }) as Box<dyn FnOnce()>)
    };

    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
    reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));

    reader.read_as_text(&file)
        .map_err(|_| ParseError::ReadError("Failed to start reading".into()))?;

    onload.forget();
    onerror.forget();

    rx.await.map_err(|_| ParseError::ReadError("Channel closed".into()))?
}

async fn parse_txt(file: File) -> Result<String, ParseError> {
    read_file_as_text(file).await
}

async fn parse_markdown(file: File) -> Result<String, ParseError> {
    let md_content = read_file_as_text(file).await?;

    let parser = Parser::new(&md_content);
    let mut text = String::new();
    let mut in_code_block = false;

    for event in parser {
        match event {
            Event::Text(t) => {
                if !in_code_block {
                    text.push_str(&t);
                    text.push(' ');
                }
            }
            Event::Start(Tag::CodeBlock(_)) => in_code_block = true,
            Event::End(TagEnd::CodeBlock) => in_code_block = false,
            Event::SoftBreak | Event::HardBreak => text.push(' '),
            _ => {}
        }
    }

    Ok(text.trim().to_string())
}

async fn parse_pdf(file: File) -> Result<String, ParseError> {
    let array_buffer = JsFuture::from(file.array_buffer())
        .await
        .map_err(|_| ParseError::ReadError("Failed to read file as array buffer".into()))?;

    let result = JsFuture::from(extract_pdf_text(&array_buffer))
        .await
        .map_err(|e| {
            let msg = e.as_string().unwrap_or_else(|| "PDF parsing failed".into());
            ParseError::ParseError(msg)
        })?;

    result
        .as_string()
        .ok_or_else(|| ParseError::ParseError("PDF extraction returned non-string".into()))
}

async fn parse_docx(file: File) -> Result<String, ParseError> {
    let array_buffer = JsFuture::from(file.array_buffer())
        .await
        .map_err(|_| ParseError::ReadError("Failed to read file as array buffer".into()))?;

    let result = JsFuture::from(extract_docx_text(&array_buffer))
        .await
        .map_err(|e| {
            let msg = e.as_string().unwrap_or_else(|| "DOCX parsing failed".into());
            ParseError::ParseError(msg)
        })?;

    result
        .as_string()
        .ok_or_else(|| ParseError::ParseError("DOCX extraction returned non-string".into()))
}

fn sanitize_content(content: &str) -> String {
    ammonia::clean(content)
}

fn extract_title(filename: &str) -> String {
    filename
        .rsplit('/')
        .next()
        .unwrap_or(filename)
        .rsplit('\\')
        .next()
        .unwrap_or(filename)
        .rsplit('.')
        .skip(1)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join(".")
}
