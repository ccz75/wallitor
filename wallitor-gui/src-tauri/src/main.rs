// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod setup;

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