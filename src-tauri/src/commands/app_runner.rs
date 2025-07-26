// App starting point for Tauri applications

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            loopback,
            stop_loopback,
            throughput,
            stop_throughput,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn loopback() -> Result<(), String> {
    super::loopback::loopback().map_err(|e| e.to_string())
}

#[tauri::command]
fn stop_loopback() -> Result<(), String> {
    super::loopback::stop_loopback().map_err(|e| e.to_string())
}

#[tauri::command]
fn throughput() -> Result<(), String> {
    super::loopback::throughput().map_err(|e| e.to_string())
}

#[tauri::command]
fn stop_throughput() -> Result<(), String> {
    super::loopback::stop_throughput().map_err(|e| e.to_string())
}