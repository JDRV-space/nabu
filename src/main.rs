mod components;
mod parser;
mod state;
mod storage;

use components::App;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
