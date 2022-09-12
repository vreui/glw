//! wayland (GNU/Linux) 支持

extern crate libc;
extern crate nix;
extern crate wayland_client;
extern crate wayland_cursor;
extern crate wayland_protocols;

#[cfg(feature = "egl")]
extern crate wayland_egl;

#[cfg(feature = "egl")]
extern crate glutin_egl_sys;

mod cursor;
mod input;
mod util;
mod wl;

pub(crate) mod 接口 {
    use std::{cell::RefCell, fs::File, rc::Rc};

    use wayland_client::{
        protocol::{wl_buffer, wl_surface},
        Main,
    };
    use wayland_protocols::xdg_shell::client::xdg_toplevel;

    use super::wl::Wl封装;
    use crate::api::{内部窗口接口, 窗口创建参数};

    // TODO 多窗口支持
    pub struct 内部窗口 {
        #[allow(dead_code)]
        非线程安全: Rc<()>,

        标题: String,
        背景色: (f32, f32, f32, f32),

        wl: Wl封装,
        运行标志: Rc<RefCell<bool>>,

        缓冲区: Main<wl_buffer::WlBuffer>,
        // 共享内存缓冲区
        缓冲区文件: File,

        表面: Main<wl_surface::WlSurface>,
        xdg顶级: Main<xdg_toplevel::XdgToplevel>,
    }

    impl 内部窗口 {
        pub fn new(参数: 窗口创建参数) -> Self {
            let mut wl = Wl封装::new();

            // 创建缓冲区
            let (缓冲区, 缓冲区文件) = wl
                .创建共享内存缓冲区((参数.大小.0 as u32, 参数.大小.1 as u32), 参数.背景色);
            // TODO 支持 EGL 缓冲区

            let 运行标志 = Rc::new(RefCell::new(false));
            // 创建窗口
            let (表面, xdg顶级) = wl.创建窗口(运行标志.clone(), 参数.标题.to_string(), &缓冲区);

            Self {
                非线程安全: Rc::new(()),

                标题: 参数.标题.to_string(),
                背景色: 参数.背景色,

                wl,
                运行标志,

                缓冲区,
                缓冲区文件,
                表面,
                xdg顶级,
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
            self.背景色
        }

        fn 设背景色(&mut self, 背景色: (f32, f32, f32, f32)) {
            self.背景色 = 背景色;
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
