use tauri::ipc::Response;
use tauri::Emitter;

#[tauri::command]
pub fn greet(name: String) -> String {
    let x = hello::add(1, 2);
    println!("start");
    std::thread::spawn({
        move || {
            println!("a");
            std::thread::sleep(std::time::Duration::from_secs(5));
            println!("b");
        }
    });
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("end");
    format!("Hello, {}! {}", name, x)
}

#[tauri::command]
pub fn my_custom_command() {
    println!("I was invoked");
}

#[tauri::command(rename_all = "snake_case")]
pub fn my_custom_msg(invoke_msg: String) {
    println!("I was invoked: {invoke_msg}");
}

#[tauri::command]
pub fn read_file() -> Response {
    let data = std::fs::read("Cargo.toml").unwrap();
    println!("{data:?}");
    tauri::ipc::Response::new(data)
}

#[tauri::command]
pub fn trigger_listen_events(app: tauri::AppHandle) {
    std::thread::spawn({
        move || {
            for i in 1..=10000 {
                app.emit("back-to-front", i).unwrap();
                // std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
    });
}
