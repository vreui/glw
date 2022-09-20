//! android 平台支持

extern crate ndk;
extern crate ndk_glue;

mod glue;
mod t;

#[cfg(feature = "egl")]
mod egl;

pub(crate) mod 接口 {
    use std::{cell::RefCell, rc::Rc};

    #[cfg(feature = "gleam")]
    use gleam::gl;

    // 导出
    #[cfg(feature = "egl")]
    pub use super::egl::Egl实现;

    use super::glue::胶水;
    use crate::api::{内部窗口接口, 窗口创建参数};

    #[cfg(feature = "egl")]
    use crate::api::Gl类型;
    #[cfg(feature = "egl")]
    use crate::egl::Egl管理器;
    #[cfg(feature = "gleam")]
    use crate::util::造绘制回调;

    pub struct 内部窗口 {
        #[allow(dead_code)]
        非线程安全: Rc<()>,

        背景色: Rc<RefCell<(f32, f32, f32, f32)>>,

        胶: 胶水,

        #[cfg(feature = "egl")]
        egl: Rc<RefCell<Option<Egl管理器>>>,
        #[cfg(feature = "gleam")]
        gl: Rc<RefCell<Option<Rc<dyn gl::Gl>>>>,
    }

    impl 内部窗口 {
        pub fn new(参数: 窗口创建参数) -> Self {
            let 背景色 = Rc::new(RefCell::new(参数.背景色));

            #[cfg(feature = "egl")]
            let egl: Rc<RefCell<Option<Egl管理器>>> = Rc::new(RefCell::new(None));
            #[cfg(feature = "gleam")]
            let gl: Rc<RefCell<Option<Rc<dyn gl::Gl>>>> = Rc::new(RefCell::new(None));

            // 绘制回调
            #[cfg(not(feature = "gleam"))]
            let 绘制回调 = Box::new(move || {
                // TODO
            });
            #[cfg(feature = "gleam")]
            let 绘制回调 = 造绘制回调(egl.clone(), gl.clone(), 背景色.clone());

            #[cfg(all(not(feature = "egl"), not(feature = "gleam")))]
            let mut 胶 = 胶水::new(绘制回调);
            #[cfg(all(feature = "egl", not(feature = "gleam")))]
            let mut 胶 = 胶水::new(egl.clone(), 绘制回调);
            #[cfg(feature = "gleam")]
            let mut 胶 = 胶水::new(egl.clone(), gl.clone(), 绘制回调);

            胶.创建窗口(参数);

            Self {
                非线程安全: Rc::new(()),
                背景色,
                胶,

                #[cfg(feature = "egl")]
                egl,
                #[cfg(feature = "gleam")]
                gl,
            }
        }
    }

    impl 内部窗口接口 for 内部窗口 {
        fn 取标题(&self) -> &str {
            // 不支持
            ""
        }

        fn 设标题(&mut self, _标题: &str) {
            // 不支持
        }

        fn 取大小(&self) -> (i32, i32) {
            // TODO
            (0, 0)
        }

        fn 设大小(&mut self, _大小: (i32, i32)) {
            // 不支持
        }

        fn 取背景色(&self) -> (f32, f32, f32, f32) {
            self.背景色.borrow().clone()
        }

        fn 设背景色(&mut self, 背景色: (f32, f32, f32, f32)) {
            self.背景色.replace(背景色);
        }

        #[cfg(feature = "egl")]
        fn 取gl类型(&self) -> Option<Gl类型> {
            self.egl.borrow().as_ref().map(|egl| egl.接口类型())
        }

        #[cfg(feature = "gleam")]
        fn 取gl(&self) -> Option<Rc<dyn gl::Gl>> {
            self.gl.borrow().as_ref().map(|g| g.clone())
        }

        fn 主循环(&mut self) {
            self.胶.主循环();
        }

        fn 清理(self) {
            // TODO
        }
    }
}
