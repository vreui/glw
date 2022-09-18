//! windows 平台支持

extern crate windows;

mod t;
mod window;

#[cfg(feature = "egl")]
mod egl;

pub(crate) mod 接口 {
    use std::{cell::RefCell, rc::Rc};

    #[cfg(feature = "gleam")]
    use gleam::gl;

    // 导出
    #[cfg(feature = "egl")]
    pub use super::egl::Egl实现;

    use super::window::窗口封装;
    use crate::api::{内部窗口接口, 窗口创建参数};

    #[cfg(feature = "egl")]
    use crate::api::Gl类型;
    #[cfg(feature = "egl")]
    use crate::egl::Egl管理器;
    #[cfg(feature = "gleam")]
    use crate::egl::初始化gleam;

    // TODO 多窗口支持
    pub struct 内部窗口 {
        #[allow(dead_code)]
        非线程安全: Rc<()>,

        背景色: Rc<RefCell<(f32, f32, f32, f32)>>,

        封装: 窗口封装,

        #[cfg(feature = "egl")]
        egl: Option<Egl管理器>,
        #[cfg(feature = "gleam")]
        gl: Option<Rc<dyn gl::Gl>>,
    }

    impl 内部窗口 {
        pub fn new(参数: 窗口创建参数) -> Self {
            let 背景色 = Rc::new(RefCell::new(参数.背景色));

            let mut 封装 = unsafe { 窗口封装::new(&参数.标题).unwrap() };

            #[cfg(feature = "egl")]
            let mut egl: Option<Egl管理器> = None;
            #[cfg(feature = "gleam")]
            let mut gl: Option<Rc<dyn gl::Gl>> = None;

            // 初始化 GL
            #[cfg(feature = "egl")]
            if 参数.gl {
                let 实现 = unsafe { 封装.初始化gl().unwrap() };
                let mut 管理器 = Egl管理器::new(实现).unwrap();

                #[cfg(feature = "gleam")]
                {
                    gl = Some(初始化gleam(&管理器));
                }
                // 设为当前
                管理器.设为当前().unwrap();

                egl = Some(管理器);
            }

            Self {
                非线程安全: Rc::new(()),
                背景色,

                封装,

                #[cfg(feature = "egl")]
                egl,
                #[cfg(feature = "gleam")]
                gl,
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

        fn 设标题(&mut self, _标题: &str) {
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
            self.背景色.borrow().clone()
        }

        fn 设背景色(&mut self, 背景色: (f32, f32, f32, f32)) {
            self.背景色.replace(背景色);
        }

        #[cfg(feature = "egl")]
        fn 取gl类型(&self) -> Option<Gl类型> {
            self.egl.as_ref().map(|egl| egl.接口类型())
        }

        #[cfg(feature = "gleam")]
        fn 取gl(&self) -> Option<Rc<dyn gl::Gl>> {
            self.gl.as_ref().map(|g| g.clone())
        }

        fn 主循环(&mut self) {
            unsafe {
                self.封装.主循环();
            }
        }

        fn 清理(self) {
            // TODO
        }
    }
}
