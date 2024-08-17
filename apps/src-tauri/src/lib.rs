use tauri::Listener;

mod commands;

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // let app_handle = app.app_handle();
            app.listen("front-to-back", move |event| {
                println!("front-to-back: {event:?}");
                // app_handle.emit(event, payload)
            });
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::my_custom_command,
            commands::my_custom_msg,
            commands::read_file,
            commands::trigger_listen_events
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
