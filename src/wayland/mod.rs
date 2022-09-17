//! wayland (GNU/Linux) 支持

extern crate nix;
extern crate wayland_client;
extern crate wayland_cursor;
extern crate wayland_protocols;

#[cfg(feature = "egl")]
extern crate wayland_egl;

mod buffer;
mod cursor;
mod input;
mod paint;
mod t;
mod util;
mod wl;
mod wlg;
mod xdgtl;

#[cfg(feature = "egl")]
mod egl;

pub(crate) mod 接口 {
    use std::{cell::RefCell, rc::Rc};

    #[cfg(feature = "gleam")]
    use gleam::gl;

    // 导出
    #[cfg(feature = "egl")]
    pub use super::egl::Egl实现;

    use super::paint::绘制参数;
    use super::t::缓冲区类型;
    use super::util::窗口默认绘制;
    use super::wl::Wl封装;
    use super::xdgtl::Xdg顶级管理器;
    use crate::api::{内部窗口接口, 窗口创建参数};

    #[cfg(feature = "egl")]
    use crate::api::Gl类型;

    // TODO 多窗口支持
    pub struct 内部窗口 {
        #[allow(dead_code)]
        非线程安全: Rc<()>,

        运行标志: Rc<RefCell<bool>>,
        背景色: Rc<RefCell<(f32, f32, f32, f32)>>,

        wl: Wl封装,
        顶级: Xdg顶级管理器,
    }

    impl 内部窗口 {
        pub fn new(参数: 窗口创建参数) -> Self {
            let 运行标志 = Rc::new(RefCell::new(false));
            let 背景色 = Rc::new(RefCell::new(参数.背景色));

            let mut wl = Wl封装::new();

            // 窗口绘制回调
            let 背景色1 = 背景色.clone();
            let 绘制回调 = Box::new(move |参数: 绘制参数| {
                窗口默认绘制(参数, 背景色1.borrow().clone());
            });

            #[cfg(feature = "egl")]
            let 缓冲类型 = if 参数.gl {
                缓冲区类型::EGL
            } else {
                缓冲区类型::共享内存
            };
            #[cfg(not(feature = "egl"))]
            let 缓冲类型 = 缓冲区类型::共享内存;

            // 创建窗口
            let 顶级 = wl.创建窗口(
                运行标志.clone(),
                (参数.大小.0 as f32, 参数.大小.1 as f32),
                参数.标题.to_string(),
                缓冲类型,
                绘制回调,
            );

            Self {
                非线程安全: Rc::new(()),
                运行标志,
                背景色,
                wl,
                顶级,
            }
        }
    }

    //impl !Send for 内部窗口 {}
    //impl !Sync for 内部窗口 {}

    impl 内部窗口接口 for 内部窗口 {
        fn 取标题(&self) -> &str {
            // TODO
            ""
        }

        fn 设标题(&mut self, 标题: &str) {
            self.顶级.设标题(标题.to_string());
        }

        fn 取大小(&self) -> (i32, i32) {
            self.wl.取大小()
        }

        fn 设大小(&mut self, _大小: (i32, i32)) {
            // wayland 不支持设大小
        }

        fn 取背景色(&self) -> (f32, f32, f32, f32) {
            self.背景色.borrow().clone()
        }

        fn 设背景色(&mut self, 背景色: (f32, f32, f32, f32)) {
            self.背景色.replace(背景色);
        }

        #[cfg(feature = "egl")]
        fn 取gl类型(&self) -> Option<Gl类型> {
            self.wl.取gl类型()
        }

        #[cfg(feature = "gleam")]
        fn 取gl(&self) -> Option<Rc<dyn gl::Gl>> {
            self.wl.取gl()
        }

        fn 主循环(&mut self) {
            self.运行标志.replace(true);

            // TODO
            while self.运行标志.borrow().clone() {
                if !self.wl.分发事件() {
                    break;
                }
            }

            self.运行标志.replace(false);
        }

        fn 清理(self) {
            self.wl.清理();
        }
    }
}
