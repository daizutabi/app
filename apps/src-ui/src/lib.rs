mod app;
mod components;

use app::App;
use leptos::prelude::*;

pub fn run() {
    // #[cfg(debug_assertions)]
    // tracing::enable();

    console_log::init_with_level(log::Level::Debug).expect("error initializing logger");
    console_error_panic_hook::set_once();

    logging::console_log("mounting to body.");

    mount_to_body(App)
}

// #[cfg(debug_assertions)]
// mod tracing {
//     use tracing::level_filters::LevelFilter;
//     use tracing_subscriber::prelude::*;
//     use tracing_web::MakeConsoleWriter;

//     const MAX_LOG_LEVEL: LevelFilter = LevelFilter::DEBUG;

//     pub fn enable() {
//         let fmt_layer = tracing_subscriber::fmt::layer()
//             .with_ansi(false) // Only partially supported across browsers
//             .pretty()
//             .without_time()
//             .with_writer(MakeConsoleWriter) // write events to the console
//             .with_filter(MAX_LOG_LEVEL);

//         tracing_subscriber::registry().with(fmt_layer).init();
//     }
// }
