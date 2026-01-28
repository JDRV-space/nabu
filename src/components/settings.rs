use leptos::prelude::*;
use crate::state::AppState;

#[component]
pub fn Settings() -> impl IntoView {
    let state = expect_context::<AppState>();
    let wpm = state.wpm;
    let chunk_size = state.chunk_size;
    let bionic_mode = state.bionic_mode;
    let punctuation_pause = state.punctuation_pause;
    let speed_ramping = state.speed_ramping;
    let theme = state.theme;
    let font_size = state.font_size;

    view! {
        <div class="settings-container glass-panel">
            <h2>"Settings"</h2>

            <section class="settings-section">
                <h3>"Reading"</h3>

                <div class="setting-item">
                    <label>"Default Speed"</label>
                    <div class="setting-control">
                        <input
                            type="range"
                            min="100"
                            max="1000"
                            step="10"
                            prop:value=move || wpm.get()
                            on:input=move |ev| {
                                let value: u32 = event_target_value(&ev).parse().unwrap_or(300);
                                state.set_wpm(value);
                            }
                        />
                        <span>{move || format!("{} WPM", wpm.get())}</span>
                    </div>
                </div>

                <div class="setting-item">
                    <label>"Word Group"</label>
                    <div class="setting-control radio-group">
                        <label class="radio">
                            <input
                                type="radio"
                                name="chunk"
                                value="1"
                                prop:checked=move || chunk_size.get() == 1
                                on:change=move |_| state.set_chunk_size(1)
                            />
                            "1"
                        </label>
                        <label class="radio">
                            <input
                                type="radio"
                                name="chunk"
                                value="3"
                                prop:checked=move || chunk_size.get() == 3
                                on:change=move |_| state.set_chunk_size(3)
                            />
                            "3"
                        </label>
                        <label class="radio">
                            <input
                                type="radio"
                                name="chunk"
                                value="5"
                                prop:checked=move || chunk_size.get() == 5
                                on:change=move |_| state.set_chunk_size(5)
                            />
                            "5"
                        </label>
                        <label class="radio">
                            <input
                                type="radio"
                                name="chunk"
                                value="10"
                                prop:checked=move || chunk_size.get() == 10
                                on:change=move |_| state.set_chunk_size(10)
                            />
                            "10"
                        </label>
                        <label class="radio">
                            <input
                                type="radio"
                                name="chunk"
                                value="20"
                                prop:checked=move || chunk_size.get() == 20
                                on:change=move |_| state.set_chunk_size(20)
                            />
                            "20"
                        </label>
                    </div>
                </div>

                <div class="setting-item">
                    <label>"Bionic Reading"</label>
                    <div class="setting-control">
                        <label class="toggle">
                            <input
                                type="checkbox"
                                prop:checked=move || bionic_mode.get()
                                on:change=move |_| state.toggle_bionic()
                            />
                            <span class="toggle-slider"></span>
                        </label>
                    </div>
                </div>

                <div class="setting-item">
                    <label>"Punctuation Pause"</label>
                    <div class="setting-control">
                        <label class="toggle">
                            <input
                                type="checkbox"
                                prop:checked=move || punctuation_pause.get()
                                on:change=move |_| state.toggle_punctuation_pause()
                            />
                            <span class="toggle-slider"></span>
                        </label>
                    </div>
                </div>

                <div class="setting-item">
                    <label>"Speed Ramping"</label>
                    <div class="setting-control">
                        <label class="toggle">
                            <input
                                type="checkbox"
                                prop:checked=move || speed_ramping.get()
                                on:change=move |_| state.toggle_speed_ramping()
                            />
                            <span class="toggle-slider"></span>
                        </label>
                    </div>
                </div>
            </section>

            <section class="settings-section">
                <h3>"Display"</h3>

                <div class="setting-item">
                    <label>"Theme"</label>
                    <div class="setting-control radio-group">
                        <label class="radio">
                            <input
                                type="radio"
                                name="theme"
                                value="dark"
                                prop:checked=move || theme.get() == "dark"
                                on:change=move |_| state.set_theme("dark".to_string())
                            />
                            "Dark"
                        </label>
                        <label class="radio">
                            <input
                                type="radio"
                                name="theme"
                                value="light"
                                prop:checked=move || theme.get() == "light"
                                on:change=move |_| state.set_theme("light".to_string())
                            />
                            "Light"
                        </label>
                    </div>
                </div>

                <div class="setting-item">
                    <label>"Font Size"</label>
                    <div class="setting-control radio-group">
                        <label class="radio">
                            <input
                                type="radio"
                                name="font_size"
                                value="S"
                                prop:checked=move || font_size.get() == "S"
                                on:change=move |_| state.set_font_size("S".to_string())
                            />
                            "S"
                        </label>
                        <label class="radio">
                            <input
                                type="radio"
                                name="font_size"
                                value="M"
                                prop:checked=move || font_size.get() == "M"
                                on:change=move |_| state.set_font_size("M".to_string())
                            />
                            "M"
                        </label>
                        <label class="radio">
                            <input
                                type="radio"
                                name="font_size"
                                value="L"
                                prop:checked=move || font_size.get() == "L"
                                on:change=move |_| state.set_font_size("L".to_string())
                            />
                            "L"
                        </label>
                        <label class="radio">
                            <input
                                type="radio"
                                name="font_size"
                                value="XL"
                                prop:checked=move || font_size.get() == "XL"
                                on:change=move |_| state.set_font_size("XL".to_string())
                            />
                            "XL"
                        </label>
                    </div>
                </div>
            </section>
        </div>
    }
}
