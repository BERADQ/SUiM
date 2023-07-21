pub fn on_created(_app: &mut tauri::App, window: tauri::Window) {
    use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

    apply_vibrancy(&window, NSVisualEffectMaterial::Sidebar, None, None)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
}
