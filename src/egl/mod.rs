//! EGL 管理器

#[cfg(feature = "gleam")]
extern crate gleam;

use core::ffi::c_void;

use std::rc::Rc;

#[cfg(feature = "gleam")]
use gleam::gl;

use crate::api::{Gl类型, Gl要求};

pub struct Egl管理器 {
    // 创建的接口类型
    类型: Gl类型,
    // TODO
}

impl Egl管理器 {
    /// 创建并初始化 EGL
    pub unsafe fn new(_要求: Gl要求, _指针: *const c_void) -> Self {
        // TODO
        Self {
            // TODO
            类型: Gl类型::Gl,
        }
    }

    /// 创建的 API 类型
    pub fn 接口类型(&self) -> Gl类型 {
        self.类型
    }

    /// 返回一个 GL 函数的地址
    pub fn 取函数地址(&self, _名称: &str) -> *const c_void {
        // TODO
        0 as *const c_void
    }

    /// egl: make_current()
    pub unsafe fn 设为当前(&mut self) -> Result<(), String> {
        // TODO
        Ok(())
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
