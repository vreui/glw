//! 缓冲区管理器

use std::{fs::File, os::unix::io::AsRawFd};

use wayland_client::{
    protocol::{wl_buffer, wl_shm},
    Main,
};

use super::util::{创建匿名文件, 填充缓冲区};

#[derive(Debug)]
pub struct 缓冲区管理器 {
    共享内存: Main<wl_shm::WlShm>,
}

impl 缓冲区管理器 {
    pub fn new(共享内存: Main<wl_shm::WlShm>) -> Self {
        Self { 共享内存 }
    }

    pub fn 取共享内存(&self) -> Main<wl_shm::WlShm> {
        self.共享内存.clone()
    }

    // wl_buffer shm
    pub fn 创建共享内存缓冲区(
        &mut self,
        大小: (u32, u32),
        颜色: (f32, f32, f32, f32),
    ) -> (Main<wl_buffer::WlBuffer>, File) {
        let mut 文件 = 创建匿名文件("shm_buffer");
        填充缓冲区(&mut 文件, 大小, 颜色);

        let 池 = self.共享内存.create_pool(
            文件.as_raw_fd(),
            (大小.0 * 大小.1 * 4) as i32, // 每像素 4 字节
        );
        let 缓冲区 = 池.create_buffer(
            0,                        // 缓冲区在池中的开始位置
            大小.0 as i32,          // 宽度 (像素)
            大小.1 as i32,          // 高度 (像素)
            (大小.0 * 4) as i32,    // 每行像素的字节数
            wl_shm::Format::Argb8888, // 像素格式
        );
        池.destroy();

        (缓冲区, 文件)
    }

    // EGL
    pub fn 创建egl缓冲区(&mut self) {
        // TODO
    }
}
