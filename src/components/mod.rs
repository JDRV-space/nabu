mod controls;
mod library;
mod reader;
mod settings;
mod stats;
mod upload;

pub use controls::Controls;
pub use library::Library;
pub use reader::Reader;
pub use settings::Settings;
pub use stats::Stats;
pub use upload::Upload;

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

use crate::state::AppState;

#[component]
pub fn App() -> impl IntoView {
    let state = AppState::new();
    provide_context(state);

    view! {
        <Router>
            <main id="app">
                <Routes fallback=|| view! { <p>"Page not found"</p> }>
                    <Route path=path!("/") view=Library />
                    <Route path=path!("/read/:id") view=Reader />
                    <Route path=path!("/stats") view=Stats />
                </Routes>
            </main>
        </Router>
    }
}
