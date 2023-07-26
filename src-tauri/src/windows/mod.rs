use window_shadows;

#[tauri::command]
pub async fn is_windows10() -> Result<bool, String> {
    let info = os_info::get();
    let os_version = info.version();
    match os_version {
        os_info::Version::Semantic(nt, _, fix) => {
            return if nt >= &10 && fix >= &22000 {
                Ok(false)
            } else {
                Ok(true)
            };
        }
        _ => {
            panic!("unknown windows version!")
        }
    }
}

pub fn on_created(_app: &mut tauri::App, window: tauri::Window) {
    window_shadows::set_shadow(&window, true).unwrap();
}
