use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use wasm_bindgen_futures::spawn_local;
use crate::state::{AppState, Document};
use crate::storage::Storage;

#[component]
pub fn Library() -> impl IntoView {
    let state = expect_context::<AppState>();
    let documents = state.documents;
    let (show_upload, set_show_upload) = signal(false);
    let (is_loading, set_loading) = signal(true);

    // Load documents from IndexedDB on mount
    Effect::new(move |_| {
        spawn_local(async move {
            let storage = Storage::new();
            match storage.get_all_documents().await {
                Ok(docs) => {
                    state.documents.set(docs);
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("Failed to load documents: {}", e).into());
                }
            }
            set_loading.set(false);
        });
    });

    let has_documents = move || !documents.get().is_empty();

    view! {
        <div class="library-container">
            <header class="library-header glass-panel">
                <h1 class="library-title">"NABU"</h1>
                <div class="header-actions">
                    <input type="text" placeholder="Search..." class="search-input" />
                    <button class="btn btn-primary" on:click=move |_| set_show_upload.set(true)>"+ Add"</button>
                </div>
            </header>

            <Show when=move || is_loading.get()>
                <div class="loading-state">
                    <div class="spinner"></div>
                    <p>"Loading library..."</p>
                </div>
            </Show>

            <Show when=move || !is_loading.get()>
                <Show
                    when=has_documents
                    fallback=move || view! { <EmptyState on_upload=move || set_show_upload.set(true) /> }
                >
                    <div class="document-grid">
                        <For
                            each=move || documents.get()
                            key=|doc| doc.id.clone()
                            children=move |doc| view! { <DocumentCard doc=doc /> }
                        />
                    </div>
                </Show>
            </Show>

            <Show when=move || show_upload.get()>
                <super::upload::UploadModal on_close=move || set_show_upload.set(false) />
            </Show>

            // TODO: Re-enable stats nav when tracking is implemented
            // <nav class="bottom-nav glass-panel">
            //     <a href="/" class="nav-item active">"Library"</a>
            //     <a href="/stats" class="nav-item">"Stats"</a>
            // </nav>
        </div>
    }
}

#[component]
fn EmptyState(on_upload: impl Fn() + Send + Sync + 'static) -> impl IntoView {
    view! {
        <div class="empty-state animate-fade-in">
            <div class="empty-icon pulse-glow">
                <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <path d="M12 3v18M5 12h14"/>
                </svg>
            </div>
            <p class="empty-text">"No documents yet"</p>
            <button class="btn btn-primary" on:click=move |_| on_upload()>
                "+ Upload your first document"
            </button>
            <p class="empty-formats">"PDF . TXT . DOCX . MD"</p>
            <div class="drop-zone">
                <p>"or drag and drop anywhere"</p>
            </div>
        </div>
    }
}

#[component]
fn DocumentCard(doc: Document) -> impl IntoView {
    let state = expect_context::<AppState>();
    let navigate = use_navigate();
    let doc_id = doc.id.clone();
    let doc_id_nav = doc.id.clone();
    let doc_id_delete = doc.id.clone();
    let doc_clone = doc.clone();
    let progress = doc.progress;
    let is_complete = progress >= 100.0;
    let is_in_progress = progress > 0.0 && progress < 100.0;
    let (show_confirm, set_show_confirm) = signal(false);

    let card_class = if is_in_progress {
        "document-card in-progress"
    } else if is_complete {
        "document-card completed"
    } else {
        "document-card"
    };

    let on_click = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
        // Set current document words for the reader
        let words: Vec<String> = doc_clone.content
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        state.current_words.set(words);
        state.current_document.set(Some(doc_clone.clone()));
        // Navigate client-side after state is set
        let path = format!("/read/{}", doc_id_nav);
        navigate(&path, Default::default());
    };

    let on_delete_click = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();
        set_show_confirm.set(true);
    };

    let last_read_display = doc.last_read.clone().unwrap_or_else(|| "Never read".to_string());
    let word_count_display = format!("{} words", doc.word_count);

    view! {
        <div class="document-card-wrapper">
            <a href=format!("/read/{}", doc_id) class=card_class on:click=on_click>
                <div class="card-progress-bar">
                    <div class="card-progress-fill" style=format!("width: {}%", progress)></div>
                </div>
                <div class="card-content">
                    <Show when=move || is_complete>
                        <span class="card-badge">"Done"</span>
                    </Show>
                    <h3 class="card-title">{doc.title.clone()}</h3>
                    <p class="card-meta">{word_count_display}</p>
                </div>
                <div class="card-footer">
                    <span class="card-progress">{format!("{}%", progress as u32)}</span>
                    <span class="card-type">{doc.file_type.clone()}</span>
                </div>
                <div class="card-date">{last_read_display}</div>
            </a>
            <button class="btn-delete" on:click=on_delete_click title="Delete document">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M3 6h18M8 6V4a2 2 0 012-2h4a2 2 0 012 2v2m3 0v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6h14"/>
                </svg>
            </button>
            {move || {
                if show_confirm.get() {
                    Some(view! { <DeleteConfirm doc_id=doc_id_delete.clone() set_show_confirm=set_show_confirm /> })
                } else {
                    None
                }
            }}
        </div>
    }
}

#[component]
fn DeleteConfirm(doc_id: String, set_show_confirm: WriteSignal<bool>) -> impl IntoView {
    let state = expect_context::<AppState>();
    let id = doc_id.clone();

    let on_confirm = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();
        let id = id.clone();
        spawn_local(async move {
            let storage = Storage::new();
            match storage.delete_document(&id).await {
                Ok(_) => {
                    match storage.get_all_documents().await {
                        Ok(docs) => state.documents.set(docs),
                        Err(e) => web_sys::console::error_1(&format!("Failed to refresh: {}", e).into()),
                    }
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("Failed to delete: {}", e).into());
                }
            }
        });
    };

    let on_cancel = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();
        set_show_confirm.set(false);
    };

    view! {
        <div class="delete-confirm">
            <p>"Delete this document?"</p>
            <div class="delete-confirm-buttons">
                <button class="btn btn-danger" on:click=on_confirm>"Delete"</button>
                <button class="btn" on:click=on_cancel>"Cancel"</button>
            </div>
        </div>
    }
}
