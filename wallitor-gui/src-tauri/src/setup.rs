use tauri::{App, Manager};
use window_vibrancy::{self, NSVisualEffectMaterial};
use window_shadows::set_shadow;


/// setup
pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let win = app.get_window("main").unwrap();
    // 仅在 macOS 下执行
    #[cfg(target_os = "macos")]
    window_vibrancy::apply_vibrancy(&win, NSVisualEffectMaterial::FullScreenUI)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    // 仅在 windows 下执行
    #[cfg(target_os = "windows")]
    if let Err(_) = window_vibrancy::apply_mica(&win, None){
        window_vibrancy::apply_acrylic(&win, Some((10,10,10,180))).expect("Unsupport Blur Effect!")
    }
    set_shadow(&win, true).unwrap();
    Ok(())
}