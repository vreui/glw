//! 对外接口定义

use std::rc::Rc;

use super::内部::内部窗口;

/// 要求创建 OpenGL / OpenGL ES 的类型
#[cfg(feature = "egl")]
#[derive(Debug, Clone, Copy)]
pub enum Gl要求 {
    /// 创建 OpenGL
    Gl {
        /// 版本号: (主, 次)
        gl版本: (u32, u32),
    },
    /// 创建 OpenGL ES
    Gles {
        /// 版本号: (主, 次)
        gles版本: (u32, u32),
    },
    /// 优先创建 OpenGL
    GlGles {
        /// 版本号: (主, 次)
        gl版本: (u32, u32),
        /// 版本号: (主, 次)
        gles版本: (u32, u32),
    },
    /// 优先创建 OpenGL ES
    GlesGl {
        /// 版本号: (主, 次)
        gl版本: (u32, u32),
        /// 版本号: (主, 次)
        gles版本: (u32, u32),
    },
}

/// 创建的 API 类型
#[cfg(feature = "egl")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gl类型 {
    /// OpenGL
    Gl,
    /// OpenGl ES
    Gles,
}

/// 基础的窗口创建参数
#[derive(Debug, Clone, Copy)]
pub struct 窗口创建参数<'a> {
    pub 标题: &'a str,
    /// 单位: 像素
    pub 大小: (i32, i32),
    /// RGBA [0.0 ~ 1.0]
    pub 背景色: (f32, f32, f32, f32),
}

impl Default for 窗口创建参数<'_> {
    fn default() -> Self {
        Self {
            标题: "",
            大小: (1280, 720),
            // 纯黑
            背景色: (0.0, 0.0, 0.0, 1.0),
        }
    }
}

impl<'a> 窗口创建参数<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn 设标题(&self, 标题: &'a str) -> Self {
        Self { 标题, ..*self }
    }

    pub fn 设大小(&self, 大小: (i32, i32)) -> Self {
        Self { 大小, ..*self }
    }

    pub fn 设背景色(&self, 背景色: (f32, f32, f32, f32)) -> Self {
        Self { 背景色, ..*self }
    }
}

/// 窗口: 各个平台的窗口抽象
///
/// 非线程安全 (仅单线程)
pub struct 窗口 {
    // 目前无法使用 `impl !Send for 窗口 {}` 的语法, 使用 Rc 标记一下
    #[allow(dead_code)]
    非线程安全: Rc<()>,

    // 内部窗口: 内部窗口接口
    内部: 内部窗口,
}

// 非线程安全
//impl !Send for 窗口 {}
//impl !Sync for 窗口 {}

impl 窗口 {
    /// 创建并初始化窗口
    ///
    /// 在创建窗口之前, 本库不会做任何初始化操作.
    pub fn new(参数: 窗口创建参数) -> Self {
        let 内部 = 内部窗口::new(参数);

        Self {
            内部,
            非线程安全: Rc::new(()),
        }
    }

    pub(crate) fn new_内部(内部: 内部窗口) -> Self {
        Self {
            内部,
            非线程安全: Rc::new(()),
        }
    }

    pub fn 取标题(&self) -> &str {
        self.内部.取标题()
    }

    pub fn 设标题(&mut self, 标题: &str) {
        self.内部.设标题(标题);
    }

    pub fn 取大小(&self) -> (i32, i32) {
        self.内部.取大小()
    }

    pub fn 设大小(&mut self, 大小: (i32, i32)) {
        self.内部.设大小(大小);
    }

    pub fn 取背景色(&self) -> (f32, f32, f32, f32) {
        self.内部.取背景色()
    }

    pub fn 设背景色(&mut self, 背景色: (f32, f32, f32, f32)) {
        self.内部.设背景色(背景色);
    }

    /// 默认主循环 (空窗口)
    ///
    /// 直到窗口关闭, 才会返回.
    pub fn 主循环(&mut self) {
        self.内部.主循环();
    }

    /// 用于销毁窗口
    pub fn 清理(self) {
        self.内部.清理();
    }
}

//pub trait 内部窗口接口: !Send + !Sync {
/// 每个平台实现的窗口功能
///
/// 非线程安全 (仅单线程): !Send + !Sync
pub trait 内部窗口接口 {
    fn 取标题(&self) -> &str;

    fn 设标题(&mut self, 标题: &str);

    fn 取大小(&self) -> (i32, i32);

    fn 设大小(&mut self, 大小: (i32, i32));

    fn 取背景色(&self) -> (f32, f32, f32, f32);

    fn 设背景色(&mut self, 背景色: (f32, f32, f32, f32));

    /// 默认主循环 (空窗口)
    ///
    /// 直到窗口关闭, 才会返回.
    fn 主循环(&mut self);

    /// 用于销毁窗口
    fn 清理(self);
}
