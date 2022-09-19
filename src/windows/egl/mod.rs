//! windows 平台的 EGL 实现

mod angle;

use std::{env, ffi, rc::Rc};

use glutin_egl_sys::egl;
use glutin_egl_sys::egl::types::EGLAttrib;

use gleam::gl;

use crate::api::{Gl类型, Gl要求};

use crate::egl::util::{
    Egl封装, 交换缓冲区, 创建显示, 创建窗口表面, 创建语境, 加载库, 取扩展, 找配置, 设为当前,
};

use super::t::ANGLE后端环境变量;

use angle::{
    Angle后端, PLATFORM_ANGLE_ANGLE, PLATFORM_ANGLE_D3D11ON12_ANGLE,
    PLATFORM_ANGLE_DEVICE_TYPE_ANGLE, PLATFORM_ANGLE_TYPE_ANGLE,
};

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

        // DEBUG
        let 扩展 = 取扩展(库, egl::NO_DISPLAY);
        println!("EGL extension: {}", 扩展);

        // ANGLE 后端环境变量
        let 环境变量 = env::var(ANGLE后端环境变量).unwrap_or("".to_string());
        // DEBUG
        if 环境变量 != "" {
            println!("ANGLE_BACKEND={}", 环境变量);
        }
        let 后端 = Angle后端::from(环境变量.as_str());

        let (显示, 配置, 语境, 表面, 类型) = {
            // ANGLE: OpenGL ES 3.0
            let (显示, 版本) = {
                let mut 属性 = Vec::<EGLAttrib>::new();
                match 后端.平台类型().unwrap() {
                    None => {}
                    Some(类型) => {
                        属性.push(PLATFORM_ANGLE_TYPE_ANGLE as EGLAttrib);
                        属性.push(类型 as EGLAttrib);

                        if 后端 == Angle后端::D3d11on12 {
                            属性.push(PLATFORM_ANGLE_D3D11ON12_ANGLE as EGLAttrib);
                            属性.push(egl::TRUE as EGLAttrib);
                        }
                    }
                }
                match 后端.设备类型() {
                    None => {}
                    Some(设备) => {
                        属性.push(PLATFORM_ANGLE_DEVICE_TYPE_ANGLE as EGLAttrib);
                        属性.push(设备 as EGLAttrib);
                    }
                }
                // 结束
                属性.push(egl::NONE as EGLAttrib);
                // DEBUG
                println!("属性 {:?}", 属性);

                // 注意: 此处应该使用 GetPlatformDisplay() 而不是 GetPlatformDisplayEXT()
                // ANGLE 官方文档中写的使用 GetPlatformDisplayEXT() 是错的
                let 平台 = (
                    // 使用 egl.GetPlatformDisplay()
                    // EGL_ANGLE_platform_angle
                    PLATFORM_ANGLE_ANGLE,
                    // 不使用 egl.GetPlatformDisplayEXT()
                    0,
                    // 不使用 egl.GetDisplay()
                    0,
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

pub fn 窗口默认绘制(g: &Rc<dyn gl::Gl>, 颜色: (f32, f32, f32, f32)) {
    let gl = gl::ErrorCheckingGl::wrap(g.clone());

    // DEBUG
    println!("颜色 {:?}", 颜色);

    // 清除背景
    gl.clear_color(颜色.0, 颜色.1, 颜色.2, 颜色.3);
    gl.clear(gl::COLOR_BUFFER_BIT);
}
