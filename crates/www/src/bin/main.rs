use leptos::prelude::*;
use leptos_router::components::Router;

use app::App;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <Router>
                <App />
            </Router>
        }
    })
}
