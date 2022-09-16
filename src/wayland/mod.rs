//! wayland (GNU/Linux) 支持

extern crate libc;
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

    // 导出
    pub use super::egl::Egl实现;

    use super::paint::绘制参数;
    use super::t::缓冲区类型;
    use super::util::窗口默认绘制;
    use super::wl::Wl封装;
    use crate::api::{内部窗口接口, 窗口创建参数};

    // TODO 多窗口支持
    pub struct 内部窗口 {
        #[allow(dead_code)]
        非线程安全: Rc<()>,

        运行标志: Rc<RefCell<bool>>,

        标题: String,
        背景色: Rc<RefCell<(f32, f32, f32, f32)>>,

        wl: Wl封装,
    }

    impl 内部窗口 {
        pub fn new(参数: 窗口创建参数) -> Self {
            let 运行标志 = Rc::new(RefCell::new(false));
            let mut wl = Wl封装::new();

            let 背景色 = Rc::new(RefCell::new(参数.背景色));

            // 窗口绘制回调
            let 背景色1 = 背景色.clone();
            let 绘制回调 = Box::new(move |参数: 绘制参数| {
                窗口默认绘制(参数, 背景色1.borrow().clone());
            });

            // 创建窗口
            wl.创建窗口(
                运行标志.clone(),
                (参数.大小.0 as f32, 参数.大小.1 as f32),
                参数.标题.to_string(),
                // TODO
                //缓冲区类型::共享内存,
                缓冲区类型::EGL,
                绘制回调,
            );

            Self {
                非线程安全: Rc::new(()),
                运行标志,

                标题: 参数.标题.to_string(),
                背景色,

                wl,
            }
        }
    }

    //impl !Send for 内部窗口 {}
    //impl !Sync for 内部窗口 {}

    impl 内部窗口接口 for 内部窗口 {
        fn 取标题(&self) -> &str {
            // TODO
            &self.标题
        }

        fn 设标题(&mut self, 标题: &str) {
            // TODO
            self.标题 = 标题.to_string();
        }

        fn 取大小(&self) -> (i32, i32) {
            // TODO
            (0, 0)
        }

        fn 设大小(&mut self, _大小: (i32, i32)) {
            // TODO
        }

        fn 取背景色(&self) -> (f32, f32, f32, f32) {
            self.背景色.borrow().clone()
        }

        fn 设背景色(&mut self, 背景色: (f32, f32, f32, f32)) {
            self.背景色.replace(背景色);
        }

        fn 主循环(&mut self) {
            self.运行标志.replace(true);

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
