//! glw: glw = glutin + winit

#[macro_use]
extern crate log;
#[macro_use]
extern crate bitflags;

extern crate instant;
extern crate mint;
extern crate once_cell;
extern crate raw_window_handle;

// 模块
mod api;
mod util;

#[cfg(target_os = "android")]
pub mod android;
#[cfg(target_os = "linux")]
pub mod wayland;
#[cfg(target_arch = "wasm32")]
pub mod web;
#[cfg(target_os = "windows")]
pub mod windows;

// 内部实现
use api::内部接口;

#[cfg(target_os = "android")]
use android::接口 as 内部;
#[cfg(target_os = "linux")]
use wayland::接口 as 内部;
#[cfg(target_arch = "wasm32")]
use web::接口 as 内部;
#[cfg(target_os = "windows")]
use windows::接口 as 内部;

// 导出
// TODO

#[cfg(test)]
mod test;
