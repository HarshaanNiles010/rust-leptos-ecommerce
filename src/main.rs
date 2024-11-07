use leptos::*;
use rust_front_end_5_leptos::App;

fn main() {
    // set up logging
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {view! { <App /> }})
}
