use leptos::prelude::*;
use leptos::ev::KeyboardEvent;
use leptos_router::hooks::use_params_map;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::prelude::*;
use web_sys::window;
use crate::state::AppState;
use crate::storage::Storage;

// Store interval ID outside reactive system - THIS IS THE FIX
thread_local! {
    static INTERVAL_ID: std::cell::Cell<Option<i32>> = const { std::cell::Cell::new(None) };
}

fn clear_global_interval() {
    INTERVAL_ID.with(|id| {
        if let Some(interval_id) = id.get() {
            if let Some(win) = window() {
                win.clear_interval_with_handle(interval_id);
            }
            id.set(None);
        }
    });
}

fn set_global_interval(new_id: i32) {
    INTERVAL_ID.with(|id| id.set(Some(new_id)));
}

fn has_global_interval() -> bool {
    INTERVAL_ID.with(|id| id.get().is_some())
}

#[component]
pub fn Reader() -> impl IntoView {
    let state = expect_context::<AppState>();
    let params = use_params_map();
    let (is_loading, set_loading) = signal(true);
    let (is_playing, set_playing) = signal(false);
    let (is_controls_visible, set_controls_visible) = signal(true);
    let (current_word_index, set_current_word_index) = signal(0usize);
    let (is_fullscreen, set_fullscreen) = signal(false);

    let words = state.current_words;
    let wpm = state.wpm;
    let font_size = state.font_size;
    let chunk_size = state.chunk_size;

    // Load document from IndexedDB
    Effect::new(move |_| {
        let doc_id = params.get().get("id").unwrap_or_default();
        if doc_id.is_empty() {
            set_loading.set(false);
            return;
        }
        spawn_local(async move {
            let storage = Storage::new();
            if let Ok(Some(doc)) = storage.get_document(&doc_id).await {
                let doc_words: Vec<String> = doc.content
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                state.current_words.set(doc_words);
                state.current_document.set(Some(doc));
            }
            set_loading.set(false);
        });
    });

    // Cleanup on unmount
    on_cleanup(move || {
        clear_global_interval();
    });

    let current_chunk = Signal::derive(move || {
        let idx = current_word_index.get();
        let chunk = chunk_size.get() as usize;
        let word_list = words.get();
        let end_idx = (idx + chunk).min(word_list.len());
        word_list.get(idx..end_idx)
            .map(|slice| slice.join(" "))
            .unwrap_or_default()
    });

    let progress_percent = move || {
        let total = words.get().len();
        if total == 0 { return 0.0; }
        (current_word_index.get() as f64 / total as f64) * 100.0
    };

    let time_remaining = move || {
        let remaining_words = words.get().len().saturating_sub(current_word_index.get());
        let wpm_val = wpm.get() as f64;
        if wpm_val == 0.0 { return "0:00".to_string(); }
        let minutes = remaining_words as f64 / wpm_val;
        let total_seconds = (minutes * 60.0) as u64;
        format!("{}:{:02}", total_seconds / 60, total_seconds % 60)
    };

    let start_playing = move || {
        if has_global_interval() { return; }

        let total = words.get_untracked().len();
        if total == 0 { return; }

        // Speed is proportional to chunk size: more words = proportionally more time
        let chunk = chunk_size.get_untracked() as u32;
        let base_delay = 60000 / wpm.get_untracked().max(1);
        let delay = base_delay * chunk;

        if let Some(win) = window() {
            let closure = Closure::<dyn Fn()>::new(move || {
                let total = words.get_untracked().len();
                let current = current_word_index.get_untracked();
                let chunk = chunk_size.get_untracked() as usize;

                let next_idx = current + chunk;
                if next_idx < total {
                    set_current_word_index.set(next_idx);
                } else {
                    clear_global_interval();
                    set_playing.set(false);
                    set_controls_visible.set(true);
                }
            });

            if let Ok(id) = win.set_interval_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                delay as i32,
            ) {
                set_global_interval(id);
                set_playing.set(true);
                set_controls_visible.set(false);
                closure.forget();
            }
        }
    };

    let stop_playing = move || {
        clear_global_interval();
        set_playing.set(false);
        set_controls_visible.set(true);
    };

    let toggle_play = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
        if is_playing.get_untracked() {
            stop_playing();
        } else {
            start_playing();
        }
    };

    let on_click = move |_| {
        if is_playing.get_untracked() {
            stop_playing();
        }
    };

    let restart = move || {
        stop_playing();
        set_current_word_index.set(0);
    };

    let toggle_fullscreen = move || {
        if let Some(win) = window() {
            if let Some(doc) = win.document() {
                if is_fullscreen.get_untracked() {
                    if doc.fullscreen_element().is_some() {
                        let _ = doc.exit_fullscreen();
                    }
                    set_fullscreen.set(false);
                } else {
                    if let Some(elem) = doc.document_element() {
                        let _ = elem.request_fullscreen();
                        set_fullscreen.set(true);
                    }
                }
            }
        }
    };

    let on_keydown = move |ev: KeyboardEvent| {
        match ev.key().as_str() {
            " " | "Space" => {
                ev.prevent_default();
                if is_playing.get_untracked() { stop_playing(); }
                else { start_playing(); }
            }
            "ArrowLeft" => { ev.prevent_default(); state.adjust_wpm(-50); }
            "ArrowRight" => { ev.prevent_default(); state.adjust_wpm(50); }
            "ArrowUp" => { ev.prevent_default(); state.adjust_wpm(10); }
            "ArrowDown" => { ev.prevent_default(); state.adjust_wpm(-10); }
            "r" | "R" => { restart(); }
            "f" | "F" => { toggle_fullscreen(); }
            "Escape" => {
                if is_fullscreen.get_untracked() {
                    toggle_fullscreen();
                } else if let Some(win) = window() {
                    let _ = win.location().set_href("/");
                }
            }
            _ => {}
        }
    };

    let font_class = Signal::derive(move || match font_size.get().as_str() {
        "S" => "word-size-s",
        "M" => "word-size-m",
        "L" => "word-size-l",
        "XL" => "word-size-xl",
        _ => "word-size-l",
    });

    view! {
        <div
            class=move || if is_fullscreen.get() { "reader-container fullscreen-mode" } else { "reader-container" }
            on:click=on_click
            on:keydown=on_keydown
            tabindex="0"
        >
            <Show when=move || is_loading.get()>
                <div class="loading-state">
                    <div class="spinner"></div>
                    <p>"Loading document..."</p>
                </div>
            </Show>

            <Show when=move || !is_loading.get()>
                <div class="reader-word-display">
                    <WordDisplay word=current_chunk font_class=font_class />
                </div>

                <Show when=move || !is_controls_visible.get()>
                    <div class="reader-minimal-progress">
                        <div class="minimal-progress-fill" style=move || format!("width: {}%", progress_percent())></div>
                    </div>
                </Show>

                <Show when=move || is_controls_visible.get()>
                    <div class="reader-controls glass-panel animate-fade-in" on:click=move |ev: web_sys::MouseEvent| ev.stop_propagation()>
                        <div class="controls-header">
                            <a href="/" class="btn">"Library"</a>
                            <div class="spacer"></div>
                            <button class="btn" on:click=move |_| toggle_fullscreen()>
                                {move || if is_fullscreen.get() { "Exit Fullscreen" } else { "Fullscreen" }}
                            </button>
                        </div>

                        <div class="controls-main">
                            <button class="btn" on:click=move |_| {
                                let current = current_word_index.get_untracked();
                                set_current_word_index.set(current.saturating_sub(50));
                            }>"|< -50"</button>
                            <button class="btn btn-primary play-btn" on:click=toggle_play>
                                {move || if is_playing.get() { "PAUSE" } else { "PLAY" }}
                            </button>
                            <button class="btn" on:click=move |_| {
                                let current = current_word_index.get_untracked();
                                let total = words.get_untracked().len();
                                set_current_word_index.set((current + 50).min(total.saturating_sub(1)));
                            }>"+50 >|"</button>
                        </div>

                        <div class="controls-wpm">
                            <button class="btn" on:click=move |_| state.adjust_wpm(-50)>"-50"</button>
                            <span class="wpm-display">{move || format!("{} WPM", wpm.get())}</span>
                            <button class="btn" on:click=move |_| state.adjust_wpm(50)>"+50"</button>
                        </div>

                        <div class="progress-section">
                            <div class="progress-bar clickable" on:click=move |ev: web_sys::MouseEvent| {
                                let target = ev.current_target().unwrap();
                                let elem = target.unchecked_ref::<web_sys::HtmlElement>();
                                let x = ev.offset_x() as f64;
                                let width = elem.offset_width() as f64;
                                if width > 0.0 {
                                    let percent = (x / width).clamp(0.0, 1.0);
                                    let total = words.get_untracked().len();
                                    let new_index = ((total as f64) * percent) as usize;
                                    set_current_word_index.set(new_index.min(total.saturating_sub(1)));
                                }
                            }>
                                <div class="progress-bar-fill" style=move || format!("width: {}%", progress_percent())></div>
                            </div>
                            <div class="progress-info">
                                <span>"Word " {move || current_word_index.get() + 1} " of " {move || words.get().len()}</span>
                                <span>{time_remaining} " remaining"</span>
                            </div>
                        </div>

                        <div class="font-size-controls">
                            <span>"Font:"</span>
                            <button class=move || if font_size.get() == "S" { "btn btn-active" } else { "btn" } on:click=move |_| state.set_font_size("S".to_string())>"S"</button>
                            <button class=move || if font_size.get() == "M" { "btn btn-active" } else { "btn" } on:click=move |_| state.set_font_size("M".to_string())>"M"</button>
                            <button class=move || if font_size.get() == "L" { "btn btn-active" } else { "btn" } on:click=move |_| state.set_font_size("L".to_string())>"L"</button>
                            <button class=move || if font_size.get() == "XL" { "btn btn-active" } else { "btn" } on:click=move |_| state.set_font_size("XL".to_string())>"XL"</button>
                        </div>

                        <div class="word-group-controls">
                            <span>"Words:"</span>
                            <button class=move || if chunk_size.get() == 1 { "btn btn-active" } else { "btn" } on:click=move |_| state.set_chunk_size(1)>"1"</button>
                            <button class=move || if chunk_size.get() == 3 { "btn btn-active" } else { "btn" } on:click=move |_| state.set_chunk_size(3)>"3"</button>
                            <button class=move || if chunk_size.get() == 5 { "btn btn-active" } else { "btn" } on:click=move |_| state.set_chunk_size(5)>"5"</button>
                            <button class=move || if chunk_size.get() == 10 { "btn btn-active" } else { "btn" } on:click=move |_| state.set_chunk_size(10)>"10"</button>
                            <button class=move || if chunk_size.get() == 20 { "btn btn-active" } else { "btn" } on:click=move |_| state.set_chunk_size(20)>"20"</button>
                        </div>

                        <div class="keyboard-hints">
                            <span>"SPACE pause"</span>
                            <span>"Arrows +/-WPM"</span>
                            <span>"R restart"</span>
                            <span>"F fullscreen"</span>
                            <span>"ESC exit"</span>
                        </div>
                    </div>
                </Show>
            </Show>
        </div>
    }
}

fn get_orp_index(len: usize) -> usize {
    match len {
        0 => 0,
        1 => 0,
        2..=5 => 1,
        6..=9 => 2,
        10..=13 => 3,
        _ => 4,
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&#39;")
}

fn format_word_with_orp(word: &str) -> (String, String, String) {
    let chars: Vec<char> = word.chars().collect();
    let orp = get_orp_index(chars.len());
    let before: String = chars.iter().take(orp).collect();
    let highlight = chars.get(orp).map(|c| c.to_string()).unwrap_or_default();
    let after: String = chars.iter().skip(orp + 1).collect();
    (html_escape(&before), html_escape(&highlight), html_escape(&after))
}

#[component]
fn WordDisplay(word: Signal<String>, font_class: Signal<&'static str>) -> impl IntoView {
    let state = expect_context::<AppState>();
    let chunk_size = state.chunk_size;

    let orp_index = Signal::derive(move || get_orp_index(word.get().chars().count()));

    view! {
        <div class=move || format!("word-container {}", font_class.get())>
            <Show when=move || chunk_size.get() == 1>
                <div class="orp-guides"><div class="orp-line-top"></div></div>
                <div class="word-text">
                    <span class="word-before">{move || word.get().chars().take(orp_index.get()).collect::<String>()}</span>
                    <span class="word-orp">{move || word.get().chars().nth(orp_index.get()).map(|c| c.to_string()).unwrap_or_default()}</span>
                    <span class="word-after">{move || word.get().chars().skip(orp_index.get() + 1).collect::<String>()}</span>
                </div>
                <div class="orp-guides"><div class="orp-line-bottom"></div></div>
            </Show>
            <Show when=move || { chunk_size.get() > 1 }>
                <div class="word-text chunk-display" inner_html=move || {
                    word.get()
                        .split_whitespace()
                        .map(|w| {
                            let (before, highlight, after) = format_word_with_orp(w);
                            format!(
                                "<span class=\"word-chunk-item\"><span class=\"word-before\">{}</span><span class=\"word-orp\">{}</span><span class=\"word-after\">{}</span></span>",
                                before, highlight, after
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("")
                }></div>
            </Show>
        </div>
    }
}
