//! glw: glw = glutin + winit

// feature 依赖处理

// gleam 依赖 egl
#[cfg(all(feature = "gleam", not(feature = "egl")))]
compile_error!("feature gleam 依赖 egl");

//#![feature(negative_impls)]

#[allow(unused_imports)]
#[macro_use]
extern crate log;
#[allow(unused_imports)]
#[macro_use]
extern crate bitflags;

extern crate instant;
extern crate mint;
extern crate once_cell;
extern crate raw_window_handle;

#[cfg(feature = "egl")]
extern crate libloading;

// 模块
mod api;
mod util;

#[cfg(feature = "egl")]
mod egl;

#[cfg(target_os = "android")]
pub mod android;
#[cfg(target_os = "linux")]
pub mod wayland;
#[cfg(target_arch = "wasm32")]
pub mod web;
#[cfg(target_os = "windows")]
pub mod windows;

// 内部实现
#[cfg(target_os = "windows")]
use crate::windows::接口 as 内部;
#[cfg(target_os = "android")]
use android::接口 as 内部;
#[cfg(target_os = "linux")]
use wayland::接口 as 内部;
#[cfg(target_arch = "wasm32")]
use web::接口 as 内部;

// 重新导出
#[cfg(target_os = "android")]
pub use ndk_glue;

// 导出
pub use api::{窗口, 窗口创建参数};

#[cfg(feature = "egl")]
pub use api::{Gl类型, Gl要求};
#[cfg(feature = "egl")]
pub use egl::Egl管理器;

// TODO

#[cfg(test)]
mod test;
