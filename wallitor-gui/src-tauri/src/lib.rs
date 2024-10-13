mod setup;
mod reader;

use std::{fs, path};
use serde_json;
use std::path::Path;
use tauri::ipc::Response;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(setup::init)
    .plugin(tauri_plugin_fs::init())
    .invoke_handler(tauri::generate_handler![read_resource_dir,get_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn read_resource_dir() -> String {
  let mut file_map = reader::FileMap::new();
  let path = Path::new("./resource");
  if let Ok(false) = fs::exists(path){
    fs::create_dir(path).expect("Can't create dir");
  }
  file_map.read_resourse_directory(path).expect("Can't read dir");
  serde_json::to_string(&file_map).unwrap()
}

// remember to call `.manage(MyState::default())`
#[tauri::command]
async fn get_file(path:String) -> Response{
  let p = path::Path::new(&path);
  if let Ok(true) =  fs::exists(p){
    let data: Vec<u8> = fs::read(p).unwrap();
    return tauri::ipc::Response::new(data);
  }
  tauri::ipc::Response::new(String::from(""))
}