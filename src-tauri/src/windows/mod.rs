use os_info;
use window_shadows;
use window_vibrancy::apply_acrylic;

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
    //如果你的系统版本符合要求，请取消注释该行
    //apply_acrylic(&window, None).unwrap();
    let os = os_info::get();
    match &os.version() {
        os_info::Version::Unknown => todo!(),
        os_info::Version::Semantic(a, _, c) => {
            if a >= &10 && (c < &1903 || c > &22000) {
                apply_acrylic(&window, None);
            };
        }
        _ => {}
    }
    window_shadows::set_shadow(&window, true).unwrap();
}
