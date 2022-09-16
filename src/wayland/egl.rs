//! wayland 平台的 EGL 实现

use std::{ffi, mem};

use glutin_egl_sys::egl::types::{EGLAttrib, EGLConfig, EGLDisplay, EGLSurface, EGLint};
use glutin_egl_sys::{egl, EGLContext};

use crate::api::{Gl类型, Gl要求};

use crate::egl::loadlib::{Egl库, EGL};

pub struct Egl实现 {
    库: &'static Egl库,

    显示: EGLDisplay,
    配置: EGLConfig,

    语境: EGLContext,
    // WindowSurface
    表面: EGLSurface,

    // 创建的接口类型
    类型: Gl类型,
}

impl Egl实现 {
    pub unsafe fn new(
        _要求: Gl要求,
        显示指针: *const ffi::c_void,
        egl表面指针: *const ffi::c_void,
    ) -> Result<Self, String> {
        // 加载 EGL 库
        let 库 = match EGL.as_ref() {
            Some(库) => 库,
            None => return Err("无法加载 EGL 库".to_string()),
        };

        let 显示 = 创建显示(库, 显示指针)?;

        let 可用配置 = 找配置(库, 显示, 配置模板::default())?;
        // TODO 检查配置
        // 使用配置
        let 配置 = 可用配置[0];

        let 语境 = 创建语境(库, 显示, 配置)?;
        let 表面 = 创建窗口表面(库, 显示, 配置, egl表面指针)?;

        Ok(Self {
            库,
            显示,
            配置,

            语境,
            表面,

            // TODO
            类型: Gl类型::Gl,
        })
    }

    pub fn 接口类型(&self) -> Gl类型 {
        self.类型
    }

    pub unsafe fn 设为当前(&mut self) -> Result<(), String> {
        if self
            .库
            .MakeCurrent(self.显示, self.表面, self.表面, self.语境)
            == egl::FALSE
        {
            let 错误码 = self.库.GetError();
            return Err(format!("无法 egl.MakeCurrent()  [{}]", 错误码));
        }

        Ok(())
    }

    pub fn 交换缓冲区(&mut self) -> Result<(), String> {
        unsafe {
            if self.库.SwapBuffers(self.显示, self.表面) == egl::FALSE {
                let 错误码 = self.库.GetError();
                return Err(format!("无法 egl.SwapBuffers()  [{}]", 错误码));
            }
        }

        Ok(())
    }
}

// EGL 配置模板
#[derive(Debug)]
struct 配置模板 {
    // TODO
}

impl Default for 配置模板 {
    fn default() -> Self {
        Self {}
    }
}

// 创建 EGLDisplay 并初始化 EGL 库
unsafe fn 创建显示(
    库: &Egl库, 显示指针: *const ffi::c_void
) -> Result<EGLDisplay, String> {
    let 显示 = if 库.GetPlatformDisplay.is_loaded() {
        // get_platform_display()
        // EGL_KHR_platform_wayland
        let 平台 = egl::PLATFORM_WAYLAND_KHR;

        let mut 属性 = Vec::<EGLAttrib>::new();
        属性.push(egl::NONE as EGLAttrib);

        库.GetPlatformDisplay(平台, 显示指针 as *mut _, 属性.as_ptr())
    } else if 库.GetPlatformDisplayEXT.is_loaded() {
        // get_platform_display_ext()
        // EGL_EXT_platform_wayland
        let 平台 = egl::PLATFORM_WAYLAND_EXT;

        let mut 属性 = Vec::<EGLAttrib>::new();
        属性.push(egl::NONE as EGLAttrib);

        库.GetPlatformDisplayEXT(平台, 显示指针 as *mut _, 属性.as_ptr() as *const _)
    } else {
        return Err("无法获取 EGL display (is_loaded)".to_string());
    };

    if 显示 == egl::NO_DISPLAY {
        return Err("无法获取 EGL display (NO_DISPLAY)".to_string());
    }

    // 初始化 EGL 库
    let (mut 主, mut 次) = (0, 0);
    if 库.Initialize(显示, &mut 主, &mut 次) == egl::FALSE {
        return Err("初始化 EGL 库失败 (FALSE)".to_string());
    }
    // DEBUG
    println!("EGL 版本  {} {}", 主, 次);

    Ok(显示)
}

unsafe fn 找配置(
    库: &Egl库,
    显示: EGLDisplay,
    模板: 配置模板,
) -> Result<Vec<EGLConfig>, String> {
    // TODO 增加外部可修改的配置项
    let mut 属性 = Vec::<EGLint>::new();

    // 缓冲区颜色类型
    // RGBA8888
    属性.push(egl::COLOR_BUFFER_TYPE as EGLint);
    属性.push(egl::RGB_BUFFER as EGLint);
    属性.push(egl::RED_SIZE as EGLint);
    属性.push(8 as EGLint);
    属性.push(egl::GREEN_SIZE as EGLint);
    属性.push(8 as EGLint);
    属性.push(egl::BLUE_SIZE as EGLint);
    属性.push(8 as EGLint);
    属性.push(egl::ALPHA_SIZE as EGLint);
    属性.push(8 as EGLint);
    // depth, stencil
    属性.push(egl::DEPTH_SIZE as EGLint);
    属性.push(8 as EGLint);
    属性.push(egl::STENCIL_SIZE as EGLint);
    属性.push(8 as EGLint);

    // 表面类型: 窗口
    属性.push(egl::SURFACE_TYPE as EGLint);
    let mut 表面类型 = 0;
    表面类型 |= egl::WINDOW_BIT;
    属性.push(表面类型 as EGLint);

    // caveat
    属性.push(egl::CONFIG_CAVEAT as EGLint);
    属性.push(egl::NONE as EGLint);

    // TODO 最小/最大 交换时间
    // 配置属性.push(egl::MIN_SWAP_INTERVAL as EGLint);
    // 配置属性.push(10 as EGLint);
    // 配置属性.push(egl::MAX_SWAP_INTERVAL as EGLint);
    // 配置属性.push(1000 as EGLint);

    // TODO 多重采样
    // 配置属性.push(egl::SAMPLE_BUFFERS as EGLint);
    // 配置属性.push(1 as EGLint);
    // 配置属性.push(egl::SAMPLES as EGLint);
    // 配置属性.push(4 as EGLint);

    // 请求 API 类型: OpenGL ES 3, OpenGL
    // TODO
    属性.push(egl::RENDERABLE_TYPE as EGLint);
    let mut 接口 = 0;
    接口 |= egl::OPENGL_ES3_BIT;
    接口 |= egl::OPENGL_BIT;
    属性.push(接口 as EGLint);

    // 结束
    属性.push(egl::NONE as EGLint);

    // 获取配置数
    let mut 配置数 = 0 as EGLint;
    库.GetConfigs(显示, std::ptr::null_mut(), 0, &mut 配置数);
    // DEBUG
    println!("EGL 配置数  {}", 配置数);

    let mut 可用配置: Vec<EGLConfig> = vec![mem::zeroed(); 配置数 as usize];

    let 结果 = 库.ChooseConfig(
        显示,
        属性.as_ptr(),
        可用配置.as_mut_ptr(),
        配置数 as EGLint,
        &mut 配置数,
    );
    if 结果 == egl::FALSE {
        return Err("未找到可用 EGL 配置".to_string());
    }
    可用配置.set_len(配置数 as usize);

    Ok(可用配置)
}

// 创建 EGLContext
unsafe fn 创建语境(
    库: &Egl库,
    显示: EGLDisplay,
    配置: EGLConfig,
) -> Result<EGLContext, String> {
    // EGL 版本 >= 1.5  EGL_KHR_create_context
    let mut 属性 = Vec::<EGLint>::new();

    // TODO 检测 OpenGL ES
    // OpenGL (core profile)
    属性.push(egl::CONTEXT_OPENGL_PROFILE_MASK as EGLint);
    属性.push(egl::CONTEXT_OPENGL_CORE_PROFILE_BIT as EGLint);
    // 注意: OpenGL 3.2 以上版本才支持 core profile

    // TODO 支持请求 GL 版本
    // 版本号
    属性.push(egl::CONTEXT_MAJOR_VERSION as EGLint);
    属性.push(3 as EGLint);
    属性.push(egl::CONTEXT_MINOR_VERSION as EGLint);
    属性.push(2 as EGLint);

    // 健壮性  EGL_EXT_create_context_robustness
    // let mut 标志 = 0;
    // 属性.push(egl::CONTEXT_OPENGL_RESET_NOTIFICATION_STRATEGY as EGLint);
    // 属性.push(egl::LOSE_CONTEXT_ON_RESET as EGLint);
    // 标志 |= egl::CONTEXT_OPENGL_ROBUST_ACCESS;

    // 调试
    // 属性.push(egl::CONTEXT_OPENGL_DEBUG as EGLint);
    // 属性.push(egl::TRUE as EGLint);

    // 属性.push(egl::CONTEXT_FLAGS_KHR as EGLint);
    // 属性.push(标志 as EGLint);

    // 结束
    属性.push(egl::NONE as EGLint);

    // TODO 支持 shared context

    // 绑定 API
    // TODO egl::OPENGL_ES_API
    if 库.BindAPI(egl::OPENGL_API) == egl::FALSE {
        return Err("无法绑定 OPENGL_API".to_string());
    }

    let 语境 = 库.CreateContext(显示, 配置, egl::NO_CONTEXT, 属性.as_ptr());
    if 语境 == egl::NO_CONTEXT {
        let 错误码 = 库.GetError();
        return Err(format!("无法创建 GL context (NO_CONTEXT)  [{}]", 错误码));
    }

    Ok(语境)
}

// 创建 EGLSurface  WindowSurface
unsafe fn 创建窗口表面(
    库: &Egl库,
    显示: EGLDisplay,
    配置: EGLConfig,
    egl表面指针: *const ffi::c_void,
) -> Result<EGLSurface, String> {
    let mut 属性 = Vec::<EGLAttrib>::new();
    // 单缓冲/双缓冲
    属性.push(egl::RENDER_BUFFER as EGLAttrib);
    属性.push(egl::BACK_BUFFER as EGLAttrib); // 双缓冲

    // TODO 颜色空间  EGL_KHR_gl_colorspace
    //egl::GL_COLORSPACE_SRGB
    //egl::GL_COLOR_SPACE_LINEAR

    // 结束
    属性.push(egl::NONE as EGLAttrib);

    // TODO EGL 版本 >= 1.5
    let 表面 = if 库.CreatePlatformWindowSurface.is_loaded() {
        库.CreatePlatformWindowSurface(显示, 配置, egl表面指针 as *mut _, 属性.as_ptr())
    } else {
        return Err("无法创建 WindowSurface (is_loaded)".to_string());
    };
    // TODO  CreatePlatformWindowSurfaceEXT()  CreateWindowSurface()

    if 表面 == egl::NO_SURFACE {
        return Err("无法创建 WindowSurface (NO_SURFACE)".to_string());
    }

    Ok(表面)
}
