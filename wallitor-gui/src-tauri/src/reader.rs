use std::fs;
use std::path::Path;
use serde::{ Serialize,Serializer};
use serde_json;
use std::collections::HashMap;

enum FileType {
    File(String),
    Directory(HashMap<String,FileType>)
}

impl Serialize for FileType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            FileType::File(file) => serializer.serialize_str(file),  // 文件只序列化路径
            FileType::Directory(dir) => dir.serialize(serializer),    // 目录序列化为HashMap
        }
    }
}

#[derive(Serialize)]
pub struct FileMap {
    files: HashMap<String, FileType>,  // 用于存储文件名和路径
}

impl FileMap {
    pub fn new()->FileMap{
        FileMap{
            files:HashMap::new()
        }
    }

    fn read_resource(&self,p:&Path)->std::io::Result<HashMap<String,FileType>>{
        let mut files:HashMap<String,FileType> = HashMap::new();
        for entry in fs::read_dir(p)?{
            let dir = entry?;
            let path = dir.path();
            if path.is_file() {
                files.insert(path.file_name().unwrap().to_string_lossy().to_string(), FileType::File(path.to_string_lossy().to_string()));
            }
        }
        Ok(files)
    }
    
    fn read_resourse_directory(&mut self,p:&Path)->std::io::Result<()>{
        for entry in fs::read_dir(p)?{
            let dir = entry?;
            let path = dir.path();
            if path.is_dir() {
                let resource =  self.read_resource(path.as_path())?;
                self.files.insert(path.to_string_lossy().to_string(), FileType::Directory(resource)) ;
            }
        }
        Ok(())
    }
}
