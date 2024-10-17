mod handler;
mod reader;
mod setup;
extern crate lazy_static;
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;

static VERSION: &str = "1.0.0";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(setup::init)
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let window = app.get_webview_window("main").expect("no main window");
            if let Ok(true) =  window.is_minimized() {
                let _ = window.unminimize();
            }
            let _ = window.set_focus();
        }))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .invoke_handler(tauri::generate_handler![
            handler::file::get_file,
            handler::file::read_resource_dir,
            handler::file::del_folder,
            handler::wallpaper::new_wallpaper,
            handler::apply::set_wallpaper,
            handler::wallpaper::edit_wallpaper,
            handler::apply::any_zoomed,
            handler::file::get_settings,
            handler::file::set_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
