use crate::{reader, VERSION};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::path::Path;
use tauri::ipc::Response;

#[tauri::command]
pub async fn get_file(path: String) -> Response {
    let p = Path::new(&path);
    if let Ok(true) = fs::exists(p) {
        let data: Vec<u8> = fs::read(p).unwrap();
        return tauri::ipc::Response::new(data);
    }
    tauri::ipc::Response::new(String::from(""))
}

#[tauri::command]
pub async fn read_resource_dir() -> String {
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

#[tauri::command]
pub async fn del_folder(path: String) -> bool {
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

#[derive(Serialize, Deserialize)]
pub struct Settings {
    title_bar: String,
    auto_start: bool,
    auto_pause: bool,
    version: String,
}

impl Settings {
    fn new() -> Settings {
        Settings {
            title_bar: String::from("win"),
            auto_start: false,
            auto_pause: true,
            version: String::from(VERSION),
        }
    }
}

static SETTING_PATH: &str = "./settings.json";

#[tauri::command]
pub async fn get_settings() -> String {
    let setting_path = Path::new(SETTING_PATH);
    if let Ok(file) = fs::read(setting_path) {
        let mut settings: Settings = serde_json::from_slice(&file).unwrap();
        settings.version = String::from(VERSION);
        return json!(settings).to_string();
    } else {
        set_settings(Settings::new()).await;
        return json!(Settings::new()).to_string();
    }
}

#[tauri::command]
pub async fn set_settings(settings: Settings) -> bool {
    let setting_path = Path::new(SETTING_PATH);
    if fs::write(setting_path, json!(settings).to_string()).is_ok() {
        return true;
    }
    false
}
