use tauri::ipc::Response;
use tauri::{AppHandle, Emitter};

#[tauri::command]
pub fn greet(app: AppHandle, name: String) -> String {
    let x = hello::add(1, 2);
    println!("start {app:?}");
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
pub async fn trigger_listen_events(app: AppHandle) {
    let emit = |x| app.emit("back-to-front", x).unwrap();

    for i in 1..=1000 {
        emit(Some(i));
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    emit(None);
    // std::thread::spawn({
    //     move || {
    //         for i in 1..=10000 {
    //             app.emit("back-to-front", i).unwrap();
    //             // std::thread::sleep(std::time::Duration::from_millis(500));
    //         }
    //     }
    // });
}
