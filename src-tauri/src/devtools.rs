#[tauri::command]
pub fn open_devtools(app: tauri::AppHandle, window: tauri::Window) -> Result<(), String> {
    window.open_devtools();
    Ok(())
}
