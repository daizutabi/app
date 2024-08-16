mod app;
mod components;

use app::App;
use leptos::prelude::*;

pub fn run() {
    console_log::init_with_level(log::Level::Debug).expect("error initializing logger");
    console_error_panic_hook::set_once();

    logging::console_log("mounting to body.");

    mount_to_body(App)
}
