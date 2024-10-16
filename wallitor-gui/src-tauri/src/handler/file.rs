use crate::reader;
use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};
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

#[derive(Serialize,Deserialize)]
struct settings{
    title_bar:String,
    auto_start:bool,
    auto_pause:bool
  }
  

#[tauri::command]
pub async fn get_config() -> String{
    if let Ok()
}