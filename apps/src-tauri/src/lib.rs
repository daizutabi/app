use tauri::Listener;

mod commands;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.listen("front-to-back", move |event| {
                println!("front-to-back: {event:?}");
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
