use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct AddInfo {
    name: String,
    preview: String,
    media: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Info {
    media_type: String,
    description: String,
    created: i64,
    entry_point: String,
}

#[derive(Serialize, Deserialize)]
struct Opt {
    mute: bool,
}

#[derive(Serialize, Deserialize)]
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
    mute: bool,
}

fn delete_preview(target_dir: &String) -> bool {
    if let Ok(entries) = fs::read_dir(target_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                    if file_name == "preview" {
                        if fs::remove_file(path).is_err() {
                            return false;
                        }
                    }
                }
            }
        }
    }
    return true;
}

fn copy_preview(source_file: &String, target_dir: &String) -> bool {
    let preview_path = Path::new(source_file);
    if let Ok(true) = fs::exists(preview_path) {
        if delete_preview(target_dir) {
            if let Some(ext) = preview_path.extension().and_then(|f| f.to_str()) {
                if fs::copy(
                    preview_path,
                    Path::new(&format!("{}/preview.{}", target_dir, ext)),
                )
                .is_ok()
                {
                    return true;
                }
            }
        }
    }
    return false;
}

#[tauri::command]
pub async fn edit_wallpaper(info: EditInfo) -> bool {
    let folder_path = Path::new(&info.path);
    if let Ok(true) = fs::exists(folder_path) {
        let config_path_str = format!("{}/config.json", info.path);
        let config_path = Path::new(&config_path_str);
        if let Ok(file) = fs::read(config_path) {
            let mut config: Config = serde_json::from_slice(&file).unwrap();
            config.info.description = info.description;
            config.name = info.name;
            config.option.mute = info.mute;
            if let Ok(()) = fs::write(config_path, json!(config).to_string()) {
                if !info.preview.is_empty() {
                    return copy_preview(&info.preview, &info.path);
                }
                return true;
            }
        }
    }
    false
}
