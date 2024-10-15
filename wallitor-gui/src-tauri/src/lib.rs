mod setup;
mod handler;
mod reader;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(setup::init)
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            handler::file::get_file,
            handler::file::read_resource_dir,
            handler::file::del_folder,
            handler::wallpaper::new_wallpaper,
            handler::apply::set_wallpaper,
            handler::wallpaper::edit_wallpaper,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
