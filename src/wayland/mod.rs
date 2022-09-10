//! wayland (GNU/Linux) 支持

extern crate libc;
extern crate wayland_client;
extern crate wayland_protocols;

pub mod 接口 {
    use std::rc::Rc;

    use crate::api::{内部窗口接口, 窗口创建参数};

    pub struct 内部窗口 {
        #[allow(dead_code)]
        非线程安全: Rc<()>,
        // TODO
    }

    impl 内部窗口 {
        pub fn new(_参数: 窗口创建参数) -> Self {
            // TODO
            Self {
                非线程安全: Rc::new(()),
            }
        }
    }

    //impl !Send for 内部窗口 {}
    //impl !Sync for 内部窗口 {}

    impl 内部窗口接口 for 内部窗口 {
        fn 取标题(&self) -> &str {
            // TODO
            return &"";
        }

        fn 设标题(&mut self, _标题: &'static str) {
            // TODO
        }

        fn 取大小(&self) -> (i32, i32) {
            // TODO
            (0, 0)
        }

        fn 设大小(&mut self, _大小: (i32, i32)) {
            // TODO
        }

        fn 取背景色(&self) -> (f32, f32, f32, f32) {
            // TODO
            (0.0, 0.0, 0.0, 0.0)
        }

        fn 设背景色(&mut self, _背景色: (f32, f32, f32, f32)) {
            // TODO
        }

        fn 主循环(&mut self) {
            // TODO
        }
    }
}

// TODO
