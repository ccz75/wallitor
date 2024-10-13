mod setup;
mod reader;

use std::{fs, path};
use serde_json::{self,json};
use std::path::Path;
use tauri::ipc::Response;
use serde::{Deserialize,Serialize};
use chrono::Local;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(setup::init)
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .invoke_handler(tauri::generate_handler![read_resource_dir,get_file,new_wallpaper])
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

#[derive(Deserialize)]
struct AddInfo {
  name: String,
  preview: String,
  media: String,
  description: String
}

#[derive(Serialize)]
struct Info{
  media_type:String,
  description:String,
  created:i64
}

#[derive(Serialize)]
struct Opt{
  mute:bool
}

#[derive(Serialize)]
struct Config{
  name:String,
  info:Info,
  option:Opt
}


#[tauri::command]
async fn new_wallpaper(info:AddInfo) -> String{
  let base_url = String::from("./resource");
  let current_time:i64 = Local::now().timestamp();
  let folder = format!("{}/{}",base_url,current_time);
  if fs::create_dir(Path::new(&folder)).is_err(){
    return String::from("Error creating folder.");
  }
  if let Ok(false) = fs::exists(Path::new(&info.preview)){
    return String::from("Source Image doesn't exist.");
  }
  if fs::copy(Path::new(&info.preview), Path::new(&format!("{}/preview.jpg",folder))).is_err(){
    return String::from("Error copy image.");
  }
  let config =json!( Config{
    name:info.name,
    info:Info { media_type: String::from("video"), description: info.description, created: current_time},
    option:Opt{mute:true}
  });
  if fs::write(Path::new(&format!("{}/config.json",folder)), config.to_string()).is_err(){
    return String::from("Error write config.");
  }
  if fs::create_dir(Path::new(&format!("{}/res",folder))).is_err(){
    return String::from("Error creating res folder.");
  }
  let media = Path::new(&info.media);
  if let Some(filename) = media.file_name().and_then(|f| f.to_str()){
    if fs::copy(Path::new(&info.media), Path::new(&format!("{}/res/{}",folder,filename))).is_err(){
      return String::from("Error copy media.");
    }
  }
  else{
    return String::from("Invalid media path.");
  }
  String::from("Success")
}