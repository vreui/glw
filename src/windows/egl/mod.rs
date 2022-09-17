//! windows 平台的 EGL 实现

mod angle;

use std::ffi;

use glutin_egl_sys::egl;
use glutin_egl_sys::egl::types::EGLAttrib;

use crate::api::{Gl类型, Gl要求};

use crate::egl::util::{
    Egl封装, 交换缓冲区, 创建显示, 创建窗口表面, 创建语境, 加载库, 找配置, 设为当前,
};

use angle::PLATFORM_ANGLE_ANGLE;

pub struct Egl实现 {
    封装: Egl封装,
}

impl Egl实现 {
    // 显示指针: GetDC()
    pub unsafe fn new(
        要求: Gl要求,
        显示指针: *const ffi::c_void,
        窗口指针: *const ffi::c_void,
    ) -> Result<Self, String> {
        let 库 = 加载库()?;

        let (显示, 配置, 语境, 表面, 类型) = {
            // ANGLE: OpenGL ES 3.0
            let (显示, 版本) = {
                let mut 属性 = Vec::<EGLAttrib>::new();
                属性.push(egl::NONE as EGLAttrib);

                let 平台 = (
                    // EGL_ANGLE_platform_angle
                    PLATFORM_ANGLE_ANGLE,
                    // EGL_ANGLE_platform_angle
                    PLATFORM_ANGLE_ANGLE,
                    // 使用 egl.GetDisplay()
                    1,
                );
                创建显示(库, 平台, 属性, 显示指针)?
            };
            // DEBUG
            println!("EGL 版本 {}.{}", 版本.0, 版本.1);

            // OpenGL ES 3.0
            let 配置 = 找配置(库, 显示, egl::OPENGL_ES3_BIT)?;

            let 要求 = match 要求 {
                Gl要求::Gl { .. } => {
                    return Err("windows 平台不支持 OpenGL".to_string());
                }
                Gl要求::Gles { gles版本 } => Gl要求::Gles { gles版本 },
                Gl要求::GlGles { gles版本, .. } => Gl要求::Gles { gles版本 },
                Gl要求::GlesGl { gles版本, .. } => Gl要求::Gles { gles版本 },
            };
            let (语境, 类型) = 创建语境(库, 显示, 配置, 要求)?;

            let 表面 = 创建窗口表面(库, 显示, 配置, 窗口指针)?;

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
