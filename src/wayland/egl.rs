//! wayland 平台的 EGL 实现

use std::ffi;

use crate::api::{Gl类型, Gl要求};

pub struct Egl实现 {
    // 创建的接口类型
    类型: Gl类型,
    // TODO
}

impl Egl实现 {
    pub unsafe fn new(
        要求: Gl要求,
        显示指针: *const ffi::c_void,
        表面指针: *const ffi::c_void,
    ) -> Self {
        Self {
            // TODO
            类型: Gl类型::Gl,
        }
    }

    pub fn 接口类型(&self) -> Gl类型 {
        self.类型
    }

    pub unsafe fn 设为当前(&mut self) -> Result<(), String> {
        // TODO
        Ok(())
    }
}
