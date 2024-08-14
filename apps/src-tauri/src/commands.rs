use tauri::ipc::Response;

#[tauri::command]
pub fn greet(name: &str) -> String {
    let x = hello::add(1, 2);
    format!("Hello, {}! You've been greeted from Rust! {}", name, x)
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
