//! 工具

use std::{
    ffi::CString,
    fs::File,
    io::{BufWriter, Write},
    os::unix::io::{AsRawFd, FromRawFd},
};

use nix::{
    sys::memfd::{memfd_create, MemFdCreateFlag},
    unistd::{lseek, Whence},
};

// 默认值
pub const 窗口边框宽度: i32 = 8; // 8 像素
pub const 窗口顶部宽度: i32 = 8;
pub const 最小窗口大小: (i32, i32) = (16, 16);

// 只存在于内存中的文件
//
// memfd_crate
pub fn 创建匿名文件(名称: &str) -> File {
    let 名称 = CString::new(名称).unwrap();
    let fd = memfd_create(&名称, MemFdCreateFlag::empty()).unwrap();
    unsafe { File::from_raw_fd(fd) }
}

// 纯色填充 (像素绘制)
//
// 颜色: RGBA
//
// 缓冲区像素格式: ARGB8888
pub fn 填充缓冲区(文件: &mut File, 大小: (u32, u32), 颜色: (f32, f32, f32, f32)) {
    // 从头开始写文件
    lseek(文件.as_raw_fd(), 0, Whence::SeekSet).unwrap();

    // ARGB
    fn 计算像素(颜色: (f32, f32, f32, f32)) -> [u8; 4] {
        let 最大 = 0xff as f32;
        ((((最大 * 颜色.3) as u32) << 24)
            | (((最大 * 颜色.0) as u32) << 16)
            | (((最大 * 颜色.1) as u32) << 8)
            | ((最大 * 颜色.2) as u32))
            .to_ne_bytes()
    }

    let 像素 = 计算像素(颜色);
    // 顶部使用半透明 (0.5 A)
    let 顶部像素 = 计算像素((颜色.0, 颜色.1, 颜色.2, 颜色.3 * 0.5));

    let mut 写 = BufWriter::new(文件);
    // 填充像素
    for y in 0..大小.1 {
        for _x in 0..大小.0 {
            if y < (窗口顶部宽度 as u32) {
                写.write_all(&顶部像素).unwrap();
            } else {
                写.write_all(&像素).unwrap();
            }
        }
    }

    写.flush().unwrap();
}

// 鼠标位于窗口的区域
//
// 用于移动窗口/调整窗口大小
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum 窗口区域 {
    // 用于移动窗口
    上边框,

    // 用于调整窗口大小 (边框)
    下边框,
    左边框,
    右边框,

    // 用于调整窗口大小 (四角)
    左上角,
    右下角,
    右上角,
    左下角,

    // 窗口中部区域 (不参与处理)
    内容,
}

impl 窗口区域 {
    /// 给定鼠标坐标, 返回位于哪个区域
    ///
    /// 坐标 (x, y): 鼠标的坐标
    /// 大小 (x, y): 窗口大小
    pub fn 测试(坐标: (f32, f32), 大小: (f32, f32)) -> Self {
        let 宽 = 窗口边框宽度 as f32;
        let 顶 = 窗口顶部宽度 as f32;

        let 左 = 宽;
        let 右 = 大小.0 - 宽;
        let 上 = 宽;
        let 下 = 大小.1 - 宽;

        let 在左 = 坐标.0 <= 左;
        let 在右 = 坐标.0 >= 右;
        let 在上 = 坐标.1 <= 上;
        let 在下 = 坐标.1 >= 下;

        // 内容区域
        if (!在左) && (!在右) && (!在上) && (!在下) {
            return Self::内容;
        }

        // 角
        if 在左 && 在上 {
            return Self::左上角;
        }
        if 在右 && 在上 {
            return Self::右上角;
        }
        if 在左 && 在下 {
            return Self::左下角;
        }

        // 边框
        if 在左 {
            return Self::左边框;
        }
        if 在下 && (!在右) {
            return Self::下边框;
        }
        if 在右 && (!在下) {
            return Self::右边框;
        }

        // 顶部
        if 坐标.1 <= 顶 {
            return Self::上边框;
        }

        // 默认为 右下角 (用于窗口太小)
        Self::右下角
    }
}
