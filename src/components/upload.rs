use leptos::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use crate::state::AppState;
use crate::parser::parse_file;
use crate::storage::Storage;

#[component]
pub fn Upload() -> impl IntoView {
    view! {
        <div class="upload-container">
            <UploadZone />
        </div>
    }
}

#[component]
pub fn UploadModal(on_close: impl Fn() + Clone + Send + Sync + 'static) -> impl IntoView {
    let on_close_clone = on_close.clone();

    view! {
        <div class="modal-backdrop" on:click=move |_| on_close_clone()>
            <div class="modal glass-panel animate-scale-in" on:click=|e| e.stop_propagation()>
                <div class="modal-header">
                    <h2>"Upload Document"</h2>
                    <button class="btn-close" on:click=move |_| on_close()>"X"</button>
                </div>
                <UploadZone />
            </div>
        </div>
    }
}

#[component]
fn UploadZone() -> impl IntoView {
    let state = expect_context::<AppState>();
    let (is_dragging, set_dragging) = signal(false);
    let (is_loading, set_loading) = signal(false);
    let (error, set_error) = signal::<Option<String>>(None);
    let (success, set_success) = signal::<Option<String>>(None);

    let handle_files = move |files: web_sys::FileList| {
        set_loading.set(true);
        set_error.set(None);
        set_success.set(None);

        for i in 0..files.length() {
            if let Some(file) = files.get(i) {
                let state = state.clone();
                spawn_local(async move {
                    match parse_file(file).await {
                        Ok(doc) => {
                            let doc_title = doc.title.clone();
                            let storage = Storage::new();

                            // Save to IndexedDB
                            match storage.save_document(&doc).await {
                                Ok(_) => {
                                    state.add_document(doc);
                                    set_success.set(Some(format!("Added: {}", doc_title)));
                                }
                                Err(e) => {
                                    // Still add to state even if storage fails
                                    state.add_document(doc);
                                    web_sys::console::warn_1(&format!("Storage error: {}", e).into());
                                    set_success.set(Some(format!("Added: {} (not persisted)", doc_title)));
                                }
                            }
                        }
                        Err(e) => {
                            set_error.set(Some(e.to_string()));
                        }
                    }
                    set_loading.set(false);
                });
            }
        }
    };

    let on_change = move |ev: leptos::ev::Event| {
        let target = ev.target().unwrap();
        let input: HtmlInputElement = target.unchecked_into();
        if let Some(files) = input.files() {
            handle_files(files);
        }
    };

    let on_drop = move |ev: leptos::ev::DragEvent| {
        ev.prevent_default();
        set_dragging.set(false);
        if let Some(dt) = ev.data_transfer() {
            if let Some(files) = dt.files() {
                handle_files(files);
            }
        }
    };

    let on_drag_over = move |ev: leptos::ev::DragEvent| {
        ev.prevent_default();
        set_dragging.set(true);
    };

    let on_drag_leave = move |_: leptos::ev::DragEvent| {
        set_dragging.set(false);
    };

    let zone_class = move || {
        if is_dragging.get() {
            "upload-zone dragging"
        } else {
            "upload-zone"
        }
    };

    view! {
        <div
            class=zone_class
            on:drop=on_drop
            on:dragover=on_drag_over
            on:dragleave=on_drag_leave
        >
            <Show when=move || is_loading.get()>
                <div class="upload-loading">
                    <div class="spinner"></div>
                    <p>"Processing..."</p>
                </div>
            </Show>

            <Show when=move || !is_loading.get()>
                <div class="upload-icon">
                    <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                        <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M17 8l-5-5-5 5M12 3v12"/>
                    </svg>
                </div>
                <p class="upload-text">"Drag & drop files here"</p>
                <p class="upload-or">"or"</p>
                <label class="btn btn-primary">
                    "Browse Files"
                    <input
                        type="file"
                        accept=".pdf,.txt,.docx,.md"
                        multiple
                        on:change=on_change
                        style="display: none"
                    />
                </label>
                <p class="upload-formats">"TXT, MD, PDF, DOCX supported."</p>
            </Show>

            <Show when=move || error.get().is_some()>
                <p class="upload-error">{move || error.get().unwrap_or_default()}</p>
            </Show>

            <Show when=move || success.get().is_some()>
                <p class="upload-success">{move || success.get().unwrap_or_default()}</p>
            </Show>
        </div>
    }
}
