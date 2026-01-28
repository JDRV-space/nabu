use leptos::prelude::*;
use crate::state::AppState;

#[component]
pub fn Stats() -> impl IntoView {
    let state = expect_context::<AppState>();
    let stats = state.reading_stats;

    let words_value = Signal::derive(move || format!("{}", stats.get().total_words));
    let words_change = Signal::derive(move || stats.get().words_change);

    let hours_value = Signal::derive(move || format!("{:.1}", stats.get().total_hours));
    let hours_change = Signal::derive(move || stats.get().hours_change);

    let wpm_value = Signal::derive(move || format!("{}", stats.get().avg_wpm));
    let wpm_change = Signal::derive(move || stats.get().wpm_change);

    let streak_value = Signal::derive(move || format!("{}", stats.get().streak_days));
    let streak_change = Signal::derive(move || None::<i32>);
    let streak_highlight = Signal::derive(move || stats.get().streak_days >= 7);

    view! {
        <div class="stats-container">
            <header class="stats-header glass-panel">
                <h1>"STATISTICS"</h1>
                <select class="time-filter">
                    <option value="week">"This Week"</option>
                    <option value="month">"This Month"</option>
                    <option value="all">"All Time"</option>
                </select>
            </header>

            <div class="stats-grid">
                <StatCard
                    value=words_value
                    label="words"
                    change=words_change
                    highlight=Signal::derive(|| false)
                />
                <StatCard
                    value=hours_value
                    label="hours"
                    change=hours_change
                    highlight=Signal::derive(|| false)
                />
                <StatCard
                    value=wpm_value
                    label="avg WPM"
                    change=wpm_change
                    highlight=Signal::derive(|| false)
                />
                <StatCard
                    value=streak_value
                    label="streak days"
                    change=streak_change
                    highlight=streak_highlight
                />
            </div>

            <section class="stats-section glass-panel">
                <h2>"Reading Activity"</h2>
                <div class="chart-placeholder">
                    <p class="chart-note">"Chart visualization coming soon"</p>
                </div>
            </section>

            <section class="stats-section glass-panel">
                <h2>"Speed Progression"</h2>
                <div class="chart-placeholder">
                    <p class="chart-note">"Chart visualization coming soon"</p>
                </div>
            </section>

            <nav class="bottom-nav glass-panel">
                <a href="/" class="nav-item">"Library"</a>
                <a href="/stats" class="nav-item active">"Stats"</a>
            </nav>
        </div>
    }
}

#[component]
fn StatCard(
    value: Signal<String>,
    label: &'static str,
    change: Signal<Option<i32>>,
    highlight: Signal<bool>,
) -> impl IntoView {
    let card_class = move || {
        if highlight.get() { "stat-card highlight" } else { "stat-card" }
    };

    let change_class = move || {
        match change.get() {
            Some(c) if c > 0 => "stat-change positive",
            Some(c) if c < 0 => "stat-change negative",
            _ => "stat-change",
        }
    };

    let change_text = move || {
        match change.get() {
            Some(c) if c > 0 => format!("^ {}%", c),
            Some(c) if c < 0 => format!("v {}%", c.abs()),
            _ => String::new(),
        }
    };

    view! {
        <div class=card_class>
            <span class="stat-value">{value}</span>
            <span class="stat-label">{label}</span>
            <span class=change_class>{change_text}</span>
        </div>
    }
}
