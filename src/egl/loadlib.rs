//! 加载动态链接库

extern crate libloading;
extern crate once_cell;

use std::{
    ffi,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use libloading::Library;
use once_cell::sync::{Lazy, OnceCell};

#[cfg(any(target_os = "linux", target_os = "android"))]
use libloading::os::unix as libloados;
#[cfg(target_os = "windows")]
use libloading::os::windows as libloados;
#[cfg(target_os = "windows")]
use libloading::os::windows::{Library as LibraryWin, LOAD_LIBRARY_SEARCH_DEFAULT_DIRS};

use glutin_egl_sys::egl;

pub trait 符号加载器 {
    unsafe fn 加载(库: &Library) -> Self;
}

#[derive(Clone)]
pub struct 符号包装<T> {
    符号: T,
    _库: Arc<Library>,
}

impl<T: 符号加载器> 符号包装<T> {
    pub unsafe fn new(库路径: &[&str]) -> Result<Self, ()> {
        for 路径 in 库路径 {
            #[cfg(any(target_os = "linux", target_os = "android"))]
            let 库 = Library::new(路径);

            #[cfg(target_os = "windows")]
            let 库 =
                LibraryWin::load_with_flags(路径, LOAD_LIBRARY_SEARCH_DEFAULT_DIRS).map(From::from);

            if let Ok(库) = 库 {
                return Ok(符号包装 {
                    符号: T::加载(&库),
                    _库: Arc::new(库),
                });
            }
        }

        Err(())
    }
}

impl<T> Deref for 符号包装<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.符号
    }
}

impl<T> DerefMut for 符号包装<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.符号
    }
}

// 加载 EGL 的动态链接库
pub struct Egl库(pub 符号包装<egl::Egl>);

unsafe impl Sync for Egl库 {}

impl Deref for Egl库 {
    type Target = egl::Egl;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Egl库 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl 符号加载器 for egl::Egl {
    unsafe fn 加载(库: &Library) -> Self {
        let 加载器 = move |符号: &'static str| -> *const ffi::c_void {
            let 符号名称 = ffi::CString::new(符号.as_bytes()).unwrap();
            if let Ok(符号) = 库.get(符号名称.as_bytes_with_nul()) {
                return *符号;
            }

            let egl函数地址 = EGL_取函数地址.get_or_init(|| {
                let 符号: libloading::Symbol<'_, Egl取函数地址> =
                    库.get(b"eglGetProcAddress\0").unwrap();
                符号.into_raw()
            });

            (egl函数地址)(符号名称.as_bytes_with_nul().as_ptr() as *const ffi::c_void)
        };

        Self::load_with(加载器)
    }
}

pub static EGL: Lazy<Option<Egl库>> = Lazy::new(|| {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    let 路径 = ["libEGL.so.1", "libEGL.so"];

    #[cfg(target_os = "windows")]
    let 路径 = ["libEGL.dll", "atioglxx.dll"];

    unsafe { 符号包装::new(&路径).map(Egl库).ok() }
});

type Egl取函数地址 = unsafe extern "C" fn(*const ffi::c_void) -> *const ffi::c_void;
static EGL_取函数地址: OnceCell<libloados::Symbol<Egl取函数地址>> = OnceCell::new();
