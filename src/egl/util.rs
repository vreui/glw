//! 通用代码封装

use std::{ffi, mem};

use glutin_egl_sys::egl::types::{EGLAttrib, EGLConfig, EGLDisplay, EGLSurface, EGLenum, EGLint};
use glutin_egl_sys::{egl, EGLContext};

use crate::api::{Gl类型, Gl要求};

use super::loadlib::{Egl库, EGL};

// 加载 EGL 库
pub fn 加载库() -> Result<&'static Egl库, String> {
    match EGL.as_ref() {
        Some(库) => Ok(库),
        None => Err("无法加载 EGL 库".to_string()),
    }
}

pub struct Egl封装 {
    pub 库: &'static Egl库,
    pub 显示: EGLDisplay,
    pub 配置: EGLConfig,
    pub 语境: EGLContext,
    pub 表面: EGLSurface,

    pub 类型: Gl类型,
}

pub unsafe fn 设为当前(封: &Egl封装) -> Result<(), String> {
    if 封.库.MakeCurrent(封.显示, 封.表面, 封.表面, 封.语境) == egl::FALSE {
        let 错误码 = 封.库.GetError();
        return Err(format!("无法 egl.MakeCurrent()  [{}]", 错误码));
    }
    Ok(())
}

pub unsafe fn 交换缓冲区(封: &Egl封装) -> Result<(), String> {
    if 封.库.SwapBuffers(封.显示, 封.表面) == egl::FALSE {
        let 错误码 = 封.库.GetError();
        return Err(format!("无法 egl.SwapBuffers()  [{}]", 错误码));
    }
    Ok(())
}

// 创建 EGLDisplay 并初始化 EGL 库
pub unsafe fn 创建显示(
    库: &Egl库,
    // GetPlatformDisplay(), GetPlatformDisplayEXT(), GetDisplay()
    平台: (EGLenum, EGLenum, i32),
    属性: Vec<EGLAttrib>,
    显示指针: *const ffi::c_void,
) -> Result<(EGLDisplay, (i32, i32)), String> {
    let 显示 = if 库.GetPlatformDisplay.is_loaded() && (平台.0 != 0) {
        库.GetPlatformDisplay(平台.0, 显示指针 as *mut _, 属性.as_ptr())
    } else if 库.GetPlatformDisplayEXT.is_loaded() && (平台.1 != 0) {
        库.GetPlatformDisplayEXT(平台.1, 显示指针 as *mut _, 属性.as_ptr() as *const _)
    } else if 平台.2 != 0 {
        库.GetDisplay(显示指针 as *mut _)
    } else {
        return Err("无法获取 EGLDisplay (is_loaded)".to_string());
    };

    if 显示 == egl::NO_DISPLAY {
        let 错误码 = 库.GetError();
        return Err(format!("无法获取 EGLDisplay (NO_DISPLAY)  [{}]", 错误码));
    }

    // 初始化 EGL 库
    let (mut 主, mut 次) = (0, 0);
    if 库.Initialize(显示, &mut 主, &mut 次) == egl::FALSE {
        let 错误码 = 库.GetError();
        return Err(format!("初始化 EGL 库失败 (FALSE)  [{}]", 错误码));
    }

    Ok((显示, (主, 次)))
}

// 接口: 请求 API 类型: OpenGL ES 3, OpenGL
// OPENGL_ES3_BIT
// OPENGL_BIT
pub unsafe fn 找配置(
    库: &Egl库, 显示: EGLDisplay, 接口: EGLenum
) -> Result<EGLConfig, String> {
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
    // egl::OPENGL_ES3_BIT
    // egl::OPENGL_BIT
    属性.push(egl::RENDERABLE_TYPE as EGLint);
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
        let 错误码 = 库.GetError();
        return Err(format!("未找到可用 EGL 配置 (FALSE)  [{}]", 错误码));
    }
    可用配置.set_len(配置数 as usize);

    if 可用配置.len() < 1 {
        return Err(format!("未找到可用 EGL 配置 ({})", 可用配置.len()));
    }

    // 默认采用第一个
    Ok(可用配置[0])
}

// 创建 EGLContext
pub unsafe fn 创建语境(
    库: &Egl库,
    显示: EGLDisplay,
    配置: EGLConfig,
    要求: Gl要求,
) -> Result<(EGLContext, Gl类型), String> {
    // 处理要求
    let (接口, 版本) = match 要求 {
        Gl要求::Gl { gl版本 } => (egl::OPENGL_API, gl版本),
        Gl要求::Gles { gles版本 } => (egl::OPENGL_ES_API, gles版本),
        // TODO 处理 OpenGL / OpenGL ES 优先关系
        Gl要求::GlGles { gl版本, .. } => (egl::OPENGL_API, gl版本),
        Gl要求::GlesGl { gl版本, .. } => (egl::OPENGL_API, gl版本),
    };

    // EGL 版本 >= 1.5  EGL_KHR_create_context
    let mut 属性 = Vec::<EGLint>::new();

    if 接口 == egl::OPENGL_API {
        // OpenGL (core profile)
        属性.push(egl::CONTEXT_OPENGL_PROFILE_MASK as EGLint);
        属性.push(egl::CONTEXT_OPENGL_CORE_PROFILE_BIT as EGLint);
        // 注意: OpenGL 3.2 以上版本才支持 core profile
    }

    // 版本号
    属性.push(egl::CONTEXT_MAJOR_VERSION as EGLint);
    属性.push(版本.0 as EGLint);
    属性.push(egl::CONTEXT_MINOR_VERSION as EGLint);
    属性.push(版本.1 as EGLint);

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
    if 库.BindAPI(接口) == egl::FALSE {
        let 接口名称 = match 接口 {
            egl::OPENGL_API => "OPENGL_API",
            egl::OPENGL_ES_API => "OPENGL_ES_API",
            _ => "",
        };
        return Err(format!("无法绑定 GL API ({})", 接口名称));
    }
    // TODO 尝试另一种 API

    let 语境 = 库.CreateContext(显示, 配置, egl::NO_CONTEXT, 属性.as_ptr());
    if 语境 == egl::NO_CONTEXT {
        let 错误码 = 库.GetError();
        return Err(format!("无法创建 GL context (NO_CONTEXT)  [{}]", 错误码));
    }

    let 类型 = if 接口 == egl::OPENGL_ES_API {
        Gl类型::Gles
    } else {
        Gl类型::Gl
    };

    Ok((语境, 类型))
}

// 创建 EGLSurface  WindowSurface
pub unsafe fn 创建窗口表面(
    库: &Egl库,
    显示: EGLDisplay,
    配置: EGLConfig,
    窗口指针: *const ffi::c_void,
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

    // EGL 版本 >= 1.5
    let 表面 = if 库.CreatePlatformWindowSurface.is_loaded() {
        库.CreatePlatformWindowSurface(显示, 配置, 窗口指针 as *mut _, 属性.as_ptr())
    } else {
        return Err("无法创建 WindowSurface (is_loaded)".to_string());
    };
    // TODO  CreatePlatformWindowSurfaceEXT()  CreateWindowSurface()

    if 表面 == egl::NO_SURFACE {
        let 错误码 = 库.GetError();
        return Err(format!("无法创建 WindowSurface (NO_SURFACE)  [{}]", 错误码));
    }

    Ok(表面)
}
