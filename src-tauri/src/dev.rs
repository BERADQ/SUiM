#[tauri::command]
pub fn open_devtools(_app: tauri::AppHandle, window: tauri::Window) {
    window.open_devtools()
}
