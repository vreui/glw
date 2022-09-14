//! 全局常量和类型定义

#[cfg(feature = "egl")]
use crate::api::Gl要求;

// 创建使用的 GL 版本
#[cfg(feature = "egl")]
pub const GL版本: Gl要求 = Gl要求::GlGles {
    gl版本: (3, 2),
    gles版本: (3, 0),
};

// 默认值
pub const 窗口边框宽度: i32 = 8; // 8 像素
pub const 窗口顶部宽度: i32 = 8;
pub const 最小窗口大小: (i32, i32) = (16, 16);

// 鼠标按键
// wl_pointer::Event::Button.button
pub const 鼠标左键: u32 = 0x110; // 272
pub const 鼠标右键: u32 = 0x111; // 273
pub const 鼠标中键: u32 = 0x112; // 274

// 鼠标指针图标
#[derive(Debug, Clone, Copy)]
pub enum 指针类型<'a> {
    默认,
    文本,
    链接,

    // 用于窗口边框
    移动,
    箭头左右,
    箭头上下,
    箭头左上右下,
    箭头左下右上,

    // 自定义名称
    名称(&'a str),
}

// 缓冲区 (绘制) 类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum 缓冲区类型 {
    // 使用共享内存 (wl_shm) 软件绘制
    共享内存,
    // OpenGL / OpenGL ES 绘制
    #[cfg(feature = "egl")]
    EGL,
}
