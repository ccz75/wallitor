use std::path::Path;
use std::fs;
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::{self, json};


#[derive(Deserialize)]
pub struct AddInfo {
    name: String,
    preview: String,
    media: String,
    description: String,
}

#[derive(Serialize,Deserialize)]
struct Info {
    media_type: String,
    description: String,
    created: i64,
    entry_point: String,
}

#[derive(Serialize,Deserialize)]
struct Opt {
    mute: bool,
}

#[derive(Serialize,Deserialize)]
pub struct Config {
    name: String,
    info: Info,
    option: Opt,
}

#[tauri::command]
pub async fn new_wallpaper(info: AddInfo) -> String {
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

#[derive(Deserialize)]
pub struct EditInfo {
    path: String,
    name: String,
    preview: String,
    description: String,
}

#[tauri::command]
pub async fn edit_wallpaper(title: String) -> bool {
    true
}