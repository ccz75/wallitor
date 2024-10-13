use tauri::{App, Manager};
#[cfg(target_os = "windows")]
use window_vibrancy;


/// setup
pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let win = app.get_webview_window("main").unwrap();
    // 仅在 windows 下执行
    #[cfg(target_os = "windows")]
    if let Err(_) = window_vibrancy::apply_mica(&win, None){
        window_vibrancy::apply_acrylic(&win, Some((10,10,10,210))).expect("Unsupport Blur Effect!")
    }
    Ok(())
}