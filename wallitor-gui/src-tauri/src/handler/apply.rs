use libloading::{Library, Symbol};
use std::{ffi::CString, rc::Rc, sync::Arc};

struct CoreModule<'a>{
    module:Arc<Library>,
    set_wallpaper:Arc<Symbol<'a,unsafe extern "C" fn(*const i8) -> i8>>,
    any_maximized:Arc<Symbol<'a,unsafe extern "C" fn() -> i8>>
}

impl<'a> CoreModule<'a> {
    fn new(library_path: &str) -> Result<Self, libloading::Error> {
        let lib = Arc::new(unsafe { Library::new(library_path).unwrap() });

        let set_fn:&Symbol<'a,unsafe extern "C" fn(*const i8) -> i8> = unsafe {
            &lib.get::<unsafe extern "C" fn(*const i8) -> i8>(b"set_wallpaper\0")?
        };
        let any_fn:&Symbol<'a,unsafe extern "C" fn() -> i8> = unsafe {
            &lib.get::<unsafe extern "C" fn() -> i8>(b"any_maximized\0")?
        };

        Ok(CoreModule {
            module: Arc::clone(&lib),
            set_wallpaper:Arc::new(set_fn),
            any_maximized:Arc::new(any_fn),
        })
    }
}

#[tauri::command]
pub async fn set_wallpaper(title: String) -> bool {
    let lib = unsafe { Library::new("wallitor-core.dll").unwrap() };
    type SetFn = unsafe extern "C" fn(*const i8) -> i8;
    let set: Symbol<SetFn> = unsafe { lib.get(b"set_wallpaper\0").unwrap() };
    let title = CString::new(title.to_string()).unwrap();
    unsafe {
        let res = set(title.as_ptr());
        if res == 0 {
            drop(lib);
            return false;
        };
    }
    drop(lib);
    return true;
}

#[tauri::command]
pub async fn anyZoomed()->bool{
    let Library
}
