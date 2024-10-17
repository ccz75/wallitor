use crate::handler::get_absolute_path;
use lazy_static::lazy_static;
use libloading::{self, Library};
use std::{ffi::CString, ops::Deref, sync::Arc};

struct CoreModule {
    _module: Arc<Library>,
    pub set_wallpaper: libloading::os::windows::Symbol<unsafe extern "C" fn(*const i8) -> i8>,
    pub any_maximized: libloading::os::windows::Symbol<unsafe extern "C" fn() -> i8>,
}

impl CoreModule {
    fn new(library_path: &str) -> CoreModule {
        let library_dir = get_absolute_path(&String::from(library_path));
        // 首先加载动态库
        let _module = Arc::new(unsafe { Library::new(library_dir).unwrap() });
        // 然后获取符号
        let set_wallpaper: libloading::os::windows::Symbol<unsafe extern "C" fn(*const i8) -> i8> = unsafe {
            _module
                .get::<unsafe extern "C" fn(*const i8) -> i8>(b"set_wallpaper\0")
                .unwrap()
                .into_raw()
        };
        let any_maximized: libloading::os::windows::Symbol<unsafe extern "C" fn() -> i8> = unsafe {
            _module
                .get::<unsafe extern "C" fn() -> i8>(b"any_maximized\0")
                .unwrap()
                .into_raw()
        };
        // 返回初始化的结构体
        CoreModule {
            _module,
            set_wallpaper,
            any_maximized,
        }
    }
}

lazy_static! {
    static ref core_module: CoreModule = CoreModule::new("wallitor-core.dll");
}

#[tauri::command]
pub async fn set_wallpaper(title: String) -> bool {
    let title = CString::new(title.to_string()).unwrap();
    unsafe {
        let res = core_module.set_wallpaper.deref()(title.as_ptr());
        if res == 0 {
            return false;
        };
    }
    true
}

#[tauri::command]
pub async fn any_zoomed() -> bool {
    unsafe {
        let res = core_module.any_maximized.deref()();
        if res == 0 {
            return false;
        }
    }
    true
}
