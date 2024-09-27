// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod setup;
mod reader;

use std::fs;
use serde_json;
use std::path::Path;

fn main() {
  tauri::Builder::default()
    .setup(setup::init)
    .invoke_handler(tauri::generate_handler![test_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
async fn test_command() -> String {
  return String::from("Hello World");
}

#[tauri::command]
async fn read_resource_dir() -> String {
  let mut file_map = reader::FileMap::new();
  let path = Path::new("./resource");
  fs::exists(path).is_err_and(|_|{fs::create_dir(path);return true;});
  serde_json::to_string(&file_map).unwrap()
}