use std::ffi::CString;
use libloading::{Library, Symbol};

#[tauri::command]
pub async fn set_wallpaper(title: String) -> bool {
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