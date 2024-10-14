mod reader;
mod setup;

use chrono::Local;
use libloading::{Library, Symbol};
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use std::ffi::CString;
use std::path::Path;
use std::{fs};
use tauri::ipc::Response;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(setup::init)
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            read_resource_dir,
            get_file,
            new_wallpaper,
            set_wallpaper,
            del_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn read_resource_dir() -> String {
    let mut file_map = reader::FileMap::new();
    let path = Path::new(".\\resource");
    if let Ok(false) = fs::exists(path) {
        fs::create_dir(path).expect("Can't create dir");
    }
    file_map
        .read_resourse_directory(path)
        .expect("Can't read dir");
    serde_json::to_string(&file_map).unwrap()
}

// remember to call `.manage(MyState::default())`
#[tauri::command]
async fn get_file(path: String) -> Response {
    let p = Path::new(&path);
    if let Ok(true) = fs::exists(p) {
        let data: Vec<u8> = fs::read(p).unwrap();
        return tauri::ipc::Response::new(data);
    }
    tauri::ipc::Response::new(String::from(""))
}

#[tauri::command]
async fn del_folder(path: String) -> bool {
    let p = Path::new(&path);
    if p.starts_with(".\\") {
        if p.is_dir() {
            if let Ok(true) = fs::exists(p) {
                if fs::remove_dir_all(p).is_ok() {
                    return true;
                }
            }
        }
    }
    false
}

#[derive(Deserialize)]
struct AddInfo {
    name: String,
    preview: String,
    media: String,
    description: String,
}

#[derive(Serialize)]
struct Info {
    media_type: String,
    description: String,
    created: i64,
    entry_point: String,
}

#[derive(Serialize)]
struct Opt {
    mute: bool,
}

#[derive(Serialize)]
struct Config {
    name: String,
    info: Info,
    option: Opt,
}

#[tauri::command]
async fn new_wallpaper(info: AddInfo) -> String {
    let base_url = String::from("./resource");
    let current_time: i64 = Local::now().timestamp();
    let folder = format!("{}/{}", base_url, current_time);
    if fs::create_dir_all(Path::new(&folder)).is_err() {
        return String::from("Error creating folder.");
    }
    if !info.preview.is_empty() {
        if let Ok(false) = fs::exists(Path::new(&info.preview)) {
            return String::from("Source Image doesn't exist.");
        }
        let preview = Path::new(&info.preview);
        if let Some(ext) = preview.extension().and_then(|f| f.to_str()) {
            if fs::copy(
                Path::new(&info.preview),
                Path::new(&format!("{}/preview.{}", folder, ext)),
            )
            .is_err()
            {
                return String::from("Error copy image.");
            }
        } else {
            return String::from("Invalid Image Path.");
        }
    }
    if fs::create_dir(Path::new(&format!("{}/res", folder))).is_err() {
        return String::from("Error creating res folder.");
    }
    let media = Path::new(&info.media);
    if let Some(filename) = media.file_name().and_then(|f| f.to_str()) {
        if fs::copy(
            Path::new(&info.media),
            Path::new(&format!("{}/res/{}", folder, filename)),
        )
        .is_err()
        {
            return String::from("Error copy media.");
        }
        let config = json!(Config {
            name: info.name,
            info: Info {
                media_type: String::from("video"),
                description: info.description,
                created: current_time,
                entry_point: String::from(filename)
            },
            option: Opt { mute: true }
        });
        if fs::write(
            Path::new(&format!("{}/config.json", folder)),
            config.to_string(),
        )
        .is_err()
        {
            return String::from("Error write config.");
        }
    } else {
        return String::from("Invalid media path.");
    }
    String::from("Success")
}

#[tauri::command]
async fn set_wallpaper(title: String) -> bool {
    let lib = unsafe { Library::new("wallitor-core.dll").unwrap() };
    type SetFn = unsafe extern "C" fn(*const i8) -> i8;
    let set: Symbol<SetFn> = unsafe { lib.get(b"set_wallpaper\0").unwrap() };
    let title = CString::new(title.to_string()).unwrap();
    unsafe {
        let res = set(title.as_ptr());
        if res == 0 {
            return false;
        };
    }
    return true;
}
