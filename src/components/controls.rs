use leptos::prelude::*;
use crate::state::AppState;

#[component]
pub fn Controls() -> impl IntoView {
    let state = expect_context::<AppState>();
    let wpm = state.wpm;

    view! {
        <div class="controls-container glass-panel">
            <WpmSlider wpm=wpm on_change=move |v| state.set_wpm(v) />
        </div>
    }
}

#[component]
fn WpmSlider(
    wpm: RwSignal<u32>,
    on_change: impl Fn(u32) + Send + Sync + 'static,
) -> impl IntoView {
    let on_input = move |ev: leptos::ev::Event| {
        let value: u32 = event_target_value(&ev).parse().unwrap_or(300);
        on_change(value);
    };

    view! {
        <div class="wpm-slider">
            <label class="wpm-label">"Speed"</label>
            <div class="slider-container">
                <span class="slider-min">"100"</span>
                <input
                    type="range"
                    min="100"
                    max="1000"
                    step="10"
                    prop:value=move || wpm.get()
                    on:input=on_input
                />
                <span class="slider-max">"1000"</span>
            </div>
            <span class="wpm-value">{move || format!("{} WPM", wpm.get())}</span>
        </div>
    }
}
