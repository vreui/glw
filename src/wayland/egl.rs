//! wayland 平台的 EGL 实现

use std::ffi;

use wayland_client::Display;
use wayland_egl::WlEglSurface;

use glutin_egl_sys::egl;
use glutin_egl_sys::egl::types::EGLAttrib;

use crate::api::{Gl类型, Gl要求};

use crate::egl::util::{
    Egl封装, 交换缓冲区, 创建显示, 创建窗口表面, 创建语境, 加载库, 找配置, 设为当前,
};

pub struct Egl实现 {
    封装: Egl封装,
}

impl Egl实现 {
    pub fn new(
        要求: Gl要求, 显示: &Display, egl表面: &WlEglSurface
    ) -> Result<Self, String> {
        let 库 = 加载库()?;

        let 显示指针 = 显示.c_ptr() as *const ffi::c_void;
        let egl表面指针 = egl表面.ptr() as *const ffi::c_void;

        let (显示, 配置, 语境, 表面, 类型) = unsafe {
            let (显示, 版本) = {
                let mut 属性 = Vec::<EGLAttrib>::new();
                属性.push(egl::NONE as EGLAttrib);

                let 平台 = (
                    // EGL_KHR_platform_wayland
                    egl::PLATFORM_WAYLAND_KHR,
                    // EGL_EXT_platform_wayland
                    egl::PLATFORM_WAYLAND_EXT,
                );
                创建显示(库, 平台, 属性, 显示指针)?
            };
            // DEBUG
            println!("EGL 版本 {}.{}", 版本.0, 版本.1);

            let 配置 = 找配置(库, 显示, egl::OPENGL_BIT | egl::OPENGL_ES3_BIT)?;
            let (语境, 类型) = 创建语境(库, 显示, 配置, 要求)?;

            let 表面 = 创建窗口表面(库, 显示, 配置, egl表面指针)?;

            (显示, 配置, 语境, 表面, 类型)
        };

        Ok(Self {
            封装: Egl封装 {
                库,
                显示,
                配置,
                语境,
                表面,
                类型,
            },
        })
    }

    pub fn 接口类型(&self) -> Gl类型 {
        self.封装.类型
    }

    pub fn 设为当前(&mut self) -> Result<(), String> {
        unsafe { 设为当前(&self.封装) }
    }

    pub fn 交换缓冲区(&mut self) -> Result<(), String> {
        unsafe { 交换缓冲区(&self.封装) }
    }
}
