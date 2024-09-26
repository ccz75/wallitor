use tauri::{App, Manager};
use window_vibrancy::{self, NSVisualEffectMaterial};


/// setup
pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let win = app.get_webview_window("main").unwrap();
    // 仅在 macOS 下执行
    #[cfg(target_os = "macos")]
    window_vibrancy::apply_vibrancy(&win, NSVisualEffectMaterial::FullScreenUI)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    // 仅在 windows 下执行
    #[cfg(target_os = "windows")]
    if let Err(_) = window_vibrancy::apply_mica(&win, None){
        window_vibrancy::apply_acrylic(&win, Some((10,10,10,210))).expect("Unsupport Blur Effect!")
    }
    Ok(())
}