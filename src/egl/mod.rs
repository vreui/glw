//! EGL 管理器

extern crate glutin_egl_sys;

#[cfg(feature = "gleam")]
extern crate gleam;

pub mod loadlib;

use std::{ffi, rc::Rc};

#[cfg(feature = "gleam")]
use gleam::gl;

use crate::api::Gl类型;
use crate::内部::Egl实现;

use loadlib::{Egl库, EGL};

pub struct Egl管理器 {
    egl: &'static Egl库,

    // 特定平台的 EGL 实现
    平台: Egl实现,
}

impl Egl管理器 {
    /// 创建并初始化 EGL
    pub fn new(平台: Egl实现) -> Result<Self, ()> {
        let egl = match EGL.as_ref() {
            Some(egl) => egl,
            None => return Err(()),
        };

        Ok(Self { egl, 平台 })
    }

    /// 创建的 API 类型
    pub fn 接口类型(&self) -> Gl类型 {
        self.平台.接口类型()
    }

    /// 返回一个 GL 函数的地址
    pub fn 取函数地址(&self, 名称: &str) -> *const ffi::c_void {
        let 名称 = ffi::CString::new(名称).unwrap();

        unsafe { self.egl.GetProcAddress(名称.as_ptr()) as *const _ }
    }

    /// egl: make_current()
    pub unsafe fn 设为当前(&mut self) -> Result<(), String> {
        self.平台.设为当前()
    }

    /// egl: swap_buffers()
    pub fn 交换缓冲区(&mut self) -> Result<(), String> {
        self.平台.交换缓冲区()
    }
}

#[cfg(feature = "gleam")]
pub fn 初始化gleam(egl: &Egl管理器) -> Rc<dyn gl::Gl> {
    match egl.接口类型() {
        Gl类型::Gl => unsafe {
            gl::GlFns::load_with(|符号| egl.取函数地址(符号) as *const _)
        },
        Gl类型::Gles => unsafe {
            gl::GlesFns::load_with(|符号| egl.取函数地址(符号) as *const _)
        },
    }
}
