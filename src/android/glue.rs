//! ndk_glue 对接代码

use std::{cell::RefCell, rc::Rc};

use ndk::{input_queue::InputQueue, native_window::NativeWindow};

use ndk_glue;
use ndk_glue::{Event, LockReadGuard};

#[cfg(feature = "gleam")]
use gleam::gl;

use crate::api::窗口创建参数;

#[cfg(feature = "egl")]
use super::egl::Egl实现;
#[cfg(feature = "egl")]
use super::t::GL版本;
#[cfg(feature = "egl")]
use crate::egl::Egl管理器;
#[cfg(feature = "gleam")]
use crate::egl::初始化gleam;

pub struct 胶水 {
    原生窗口: Option<LockReadGuard<NativeWindow>>,
    输入队列: Option<LockReadGuard<InputQueue>>,

    绘制回调: Box<dyn FnMut() -> () + 'static>,

    #[cfg(feature = "egl")]
    参数_gl: bool,
    #[cfg(feature = "egl")]
    egl: Rc<RefCell<Option<Egl管理器>>>,
    #[cfg(feature = "gleam")]
    gl: Rc<RefCell<Option<Rc<dyn gl::Gl>>>>,
}

impl 胶水 {
    #[cfg(all(not(feature = "egl"), not(feature = "gleam")))]
    pub fn new(绘制回调: Box<dyn FnMut() -> () + 'static>) -> Self {
        Self {
            原生窗口: None,
            输入队列: None,
            绘制回调,
        }
    }

    #[cfg(all(feature = "egl", not(feature = "gleam")))]
    pub fn new(
        egl: Rc<RefCell<Option<Egl管理器>>>,
        绘制回调: Box<dyn FnMut() -> () + 'static>,
    ) -> Self {
        Self {
            egl,
            参数_gl: false,
            原生窗口: None,
            输入队列: None,
            绘制回调,
        }
    }

    #[cfg(feature = "gleam")]
    pub fn new(
        egl: Rc<RefCell<Option<Egl管理器>>>,
        gl: Rc<RefCell<Option<Rc<dyn gl::Gl>>>>,
        绘制回调: Box<dyn FnMut() -> () + 'static>,
    ) -> Self {
        Self {
            egl,
            gl,
            参数_gl: false,
            原生窗口: None,
            输入队列: None,
            绘制回调,
        }
    }

    pub fn 创建窗口(&mut self, 参数: 窗口创建参数) {
        #[cfg(feature = "egl")]
        {
            self.参数_gl = 参数.gl;
        }

        self.处理事件(true);
    }

    pub fn 主循环(&mut self) {
        self.处理事件(false);
    }

    // Event::WindowCreated
    fn 窗口已创建(&mut self) {
        self.原生窗口 = ndk_glue::native_window();

        #[cfg(feature = "egl")]
        {
            let 原生窗口: &NativeWindow = &self.原生窗口.as_ref().unwrap();

            if self.egl.borrow().is_none() {
                // 初始化 GL
                let 实现 = Egl实现::new(GL版本, 原生窗口).unwrap();
                let mut egl = Egl管理器::new(实现).unwrap();
                // 设为当前
                egl.设为当前().unwrap();

                #[cfg(feature = "gleam")]
                {
                    // 初始化 gleam
                    let gl = 初始化gleam(&egl);
                    self.gl.replace(Some(gl));
                }

                self.egl.replace(Some(egl));
            } else {
                // 重新创建 WindowSurface
                let mut egl1 = self.egl.borrow_mut();
                let egl = egl1.as_mut().unwrap();
                egl.取实现().重新创建表面(原生窗口).unwrap();
                // 设为当前
                egl.设为当前().unwrap();
            }
        }
    }

    // Event::WindowResized
    fn 窗口改变大小(&mut self) {
        // TODO
    }

    // Event::WindowRedrawNeeded
    fn 窗口需要重绘(&mut self) {
        (self.绘制回调)();
    }

    // Event::WindowDestroyed
    fn 窗口已销毁(&mut self) {
        self.原生窗口 = None;
    }

    // Event::InputQueueCreated
    fn 输入队列已创建(&mut self) {
        self.输入队列 = ndk_glue::input_queue();
    }

    // Event::InputQueueDestroyed
    fn 输入队列已销毁(&mut self) {
        self.输入队列 = None;
    }

    fn 处理事件(&mut self, 窗口创建后退出: bool) {
        loop {
            match ndk_glue::poll_events() {
                Some(事件) => match 事件 {
                    Event::Start => {
                        println!("Event::Start");
                    }
                    Event::Resume => {
                        println!("Event::Resume");
                    }
                    Event::SaveInstanceState => {
                        println!("Event::SaveInstanceState");
                    }
                    Event::Pause => {
                        println!("Event::Pause");
                    }
                    Event::Stop => {
                        println!("Event::Stop");
                    }
                    Event::Destroy => {
                        println!("Event::Destroy");
                        // 退出程序
                        break;
                    }
                    Event::ConfigChanged => {
                        println!("Event::ConfigChanged");
                    }
                    Event::LowMemory => {
                        println!("Event::LowMemory");
                    }
                    Event::WindowLostFocus => {
                        println!("Event::WindowLostFocus");
                    }
                    Event::WindowHasFocus => {
                        println!("Event::WindowHasFocus");
                    }
                    Event::WindowCreated => {
                        println!("Event::WindowCreated");
                        self.窗口已创建();

                        if 窗口创建后退出 {
                            break;
                        }
                    }
                    Event::WindowResized => {
                        println!("Event::WindowResized");
                        self.窗口改变大小();
                    }
                    Event::WindowRedrawNeeded => {
                        println!("Event::WindowRedrawNeeded");
                        self.窗口需要重绘();
                    }
                    Event::WindowDestroyed => {
                        println!("Event::WindowDestroyed");
                        self.窗口已销毁();
                    }
                    Event::InputQueueCreated => {
                        println!("Event::InputQueueCreated");
                        self.输入队列已创建();
                    }
                    Event::InputQueueDestroyed => {
                        println!("Event::InputQueueDestroyed");
                        self.输入队列已销毁();
                    }
                    Event::ContentRectChanged => {
                        println!("Event::ContentRectChanged");
                    }
                },
                None => {}
            }
        }
    }
}
