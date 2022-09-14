//! 窗口绘制管理器

use std::fmt::{Debug, Formatter};
use std::{cell::RefCell, fs::File, rc::Rc};

#[cfg(feature = "egl")]
use wayland_egl::WlEglSurface;

#[cfg(feature = "gleam")]
use gleam::gl;

use super::buffer::缓冲区管理器;
use super::t::缓冲区类型;
use super::util::表面设置更新区域;
use super::wlg::Wl全局管理器;
use super::xdgtl::Xdg顶级管理器;

#[cfg(feature = "egl")]
use super::t::GL版本;
#[cfg(feature = "egl")]
use crate::api::Gl类型;
#[cfg(feature = "egl")]
use crate::egl::Egl管理器;
#[cfg(feature = "gleam")]
use crate::egl::初始化gleam;

// 绘制窗口用的参数
pub struct 绘制参数<'a> {
    // 当前分辨率
    pub 大小: (u32, u32),

    pub 类型: 绘制参数类型<'a>,
}

pub enum 绘制参数类型<'a> {
    共享内存(绘制参数_共享内存<'a>),

    #[cfg(feature = "egl")]
    EGL(绘制参数Egl<'a>),
}

// 缓冲区类型::EGL 专用和绘制参数
#[cfg(feature = "egl")]
pub struct 绘制参数Egl<'a> {
    // 是否初始绘制标志
    初始绘制: bool,

    #[cfg(feature = "gleam")]
    gl: &'a Rc<dyn gl::Gl>,
}

// 缓冲区类型::共享内存 专用的绘制参数
pub struct 绘制参数_共享内存<'a> {
    pub 最大大小: (u32, u32),
    // 是否首次绘制标志
    pub 已绘制: bool,
    // 每行像素间隔的字节数
    pub 行间隔: i32,
    // 缓冲区对应的文件
    pub 文件: &'a mut File,
}

//#[derive(Debug)]
pub struct 窗口绘制管理器 {
    窗口大小: Rc<RefCell<(f32, f32)>>,
    缓冲: 缓冲区管理器,

    顶级: Xdg顶级管理器,
    绘制类型: 缓冲区类型,
    绘制回调: Box<dyn FnMut(绘制参数) -> () + 'static>,

    #[cfg(feature = "egl")]
    egl表面: Option<WlEglSurface>,
    #[cfg(feature = "egl")]
    egl: Option<Egl管理器>,

    #[cfg(feature = "gleam")]
    gl: Option<Rc<dyn gl::Gl>>,
}

impl Debug for 窗口绘制管理器 {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // TODO
        Ok(())
    }
}

impl 窗口绘制管理器 {
    pub fn new(
        全局: &Wl全局管理器,
        窗口大小: Rc<RefCell<(f32, f32)>>,
        顶级: &Xdg顶级管理器,
        绘制类型: 缓冲区类型,
        绘制回调: Box<dyn FnMut(绘制参数) -> () + 'static>,
    ) -> Self {
        let 缓冲 = 缓冲区管理器::new(全局.取共享内存().clone());

        Self {
            窗口大小,
            缓冲,

            顶级: 顶级.clone(),
            绘制类型,
            绘制回调,

            #[cfg(feature = "egl")]
            egl表面: None,
            #[cfg(feature = "egl")]
            egl: None,
            #[cfg(feature = "gleam")]
            gl: None,
        }
    }

    pub fn 初始绘制(&mut self, 初始大小: (f32, f32)) {
        let 大小 = (初始大小.0 as u32, 初始大小.1 as u32);

        match self.绘制类型 {
            #[cfg(feature = "egl")]
            缓冲区类型::EGL => {
                self.初始绘制_egl(大小);
            }
            缓冲区类型::共享内存 => {
                self.初始绘制_共享内存(大小);
            }
        }
    }

    #[cfg(feature = "egl")]
    fn 初始绘制_egl(&mut self, 大小: (u32, u32)) {
        // 初始化 EGL
        let 表面 = self.顶级.取表面();
        let egl表面 = WlEglSurface::new(表面, 大小.0 as i32, 大小.1 as i32);

        let egl表面指针 = egl表面.ptr();
        let egl = unsafe { Egl管理器::new(GL版本, egl表面指针) };

        #[cfg(feature = "gleam")]
        {
            // 初始化 gleam
            let gl = 初始化gleam(&egl);
            // 绘制
            (self.绘制回调)(绘制参数 {
                大小,
                类型: 绘制参数类型::EGL(绘制参数Egl {
                    初始绘制: true,
                    gl: &gl,
                }),
            });

            表面.commit();

            self.gl = Some(gl);
        }

        self.egl表面 = Some(egl表面);
        self.egl = Some(egl);
    }

    fn 初始绘制_共享内存(&mut self, 大小: (u32, u32)) {
        let 缓冲1 = self.缓冲.取共享内存缓冲区(大小);
        let mut 缓冲 = 缓冲1.borrow_mut();

        (self.绘制回调)(绘制参数 {
            大小,
            类型: 绘制参数类型::共享内存(绘制参数_共享内存 {
                最大大小: 缓冲.取最大大小(),
                已绘制: 缓冲.取绘制标志(),
                行间隔: 缓冲.行间隔字节(),
                文件: 缓冲.取文件(),
            }),
        });
        缓冲.设绘制标志(true);

        let 表面 = self.顶级.取表面();
        // 附加缓冲区
        表面.attach(Some(缓冲.取缓冲区()), 0, 0);
        表面.commit();
    }

    // 处理窗口大小改变 (绘制)
    pub fn 改变大小(&mut self, 大小: (i32, i32)) {
        // 检查大小
        if (大小.0 <= 0) || (大小.1 <= 0) {
            return;
        }
        // 检查大小是否改变
        let 窗口大小 = self.窗口大小.borrow().clone();
        if (窗口大小.0 as i32 == 大小.0) && (窗口大小.1 as i32 == 大小.1) {
            return;
        }

        // 重新绘制
        let 大小 = (大小.0 as u32, 大小.1 as u32);

        match self.绘制类型 {
            #[cfg(feature = "egl")]
            缓冲区类型::EGL => {
                self.重新绘制_egl(大小);
            }
            缓冲区类型::共享内存 => {
                self.重新绘制_共享内存(大小);
            }
        }
    }

    #[cfg(feature = "egl")]
    fn 重新绘制_egl(&mut self, 大小: (u32, u32)) {
        // 更新 EGL 表面大小
        self.egl表面
            .as_ref()
            .unwrap()
            .resize(大小.0 as i32, 大小.1 as i32, 0, 0);

        let 表面 = self.顶级.取表面();
        // 绘制
        #[cfg(feature = "gleam")]
        {
            let gl = self.gl.as_ref().unwrap();
            (self.绘制回调)(绘制参数 {
                大小,
                类型: 绘制参数类型::EGL(绘制参数Egl {
                    初始绘制: false,
                    gl,
                }),
            });

            表面设置更新区域(表面, (0, 0, 大小.0 as i32, 大小.1 as i32));
        }

        表面.commit();
    }

    fn 重新绘制_共享内存(&mut self, 大小: (u32, u32)) {
        let 缓冲1 = self.缓冲.取共享内存缓冲区(大小);
        let mut 缓冲 = 缓冲1.borrow_mut();

        (self.绘制回调)(绘制参数 {
            大小,
            类型: 绘制参数类型::共享内存(绘制参数_共享内存 {
                最大大小: 缓冲.取最大大小(),
                已绘制: 缓冲.取绘制标志(),
                行间隔: 缓冲.行间隔字节(),
                文件: 缓冲.取文件(),
            }),
        });
        缓冲.设绘制标志(true);

        let 表面 = self.顶级.取表面();
        // 附加缓冲区
        表面.attach(Some(缓冲.取缓冲区()), 0, 0);
        表面设置更新区域(表面, (0, 0, 大小.0 as i32, 大小.1 as i32));
        表面.commit();
    }
}
