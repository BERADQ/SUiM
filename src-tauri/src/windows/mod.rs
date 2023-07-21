use window_vibrancy::{apply_blur, apply_mica};

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
    let info = os_info::get();
    let os_version = info.version();
    match os_version {
        os_info::Version::Semantic(nt, _, fix) => {
            if nt >= &10 && fix >= &22000 {
                apply_mica(&window, None).unwrap();
            } else if nt >= &10 {
                // nothing
            } else {
                panic!("unknown windows version! info: {}", info)
            }
        }
        _ => {
            panic!("unknown windows version!")
        }
    }

    window.set_decorations(true).unwrap();
}
