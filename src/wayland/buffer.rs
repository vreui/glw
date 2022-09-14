//! 缓冲区管理器 (共享内存)

use std::{cell::RefCell, fs::File, os::unix::io::AsRawFd, rc::Rc};

use wayland_client::{
    protocol::{wl_buffer, wl_shm, wl_shm_pool},
    Main,
};

use super::util::创建匿名文件;

#[derive(Debug)]
struct 共享内存双缓冲 {
    pub 缓冲1: Rc<RefCell<共享内存缓冲区>>,
    pub 缓冲2: Rc<RefCell<共享内存缓冲区>>,
    // 当前使用的是哪个缓冲区
    // false: 缓冲1, true: 缓冲2
    pub 标志: Rc<RefCell<bool>>,
}

#[derive(Debug)]
pub struct 缓冲区管理器 {
    共享内存: Main<wl_shm::WlShm>,
    // 共享内存缓冲区: 内部采用双缓冲区
    共享内存缓冲: Option<共享内存双缓冲>,
}

impl 缓冲区管理器 {
    pub fn new(共享内存: Main<wl_shm::WlShm>) -> Self {
        Self {
            共享内存,
            共享内存缓冲: None,
        }
    }

    fn 共享内存缓冲区检查大小(
        &self,
        缓冲: Rc<RefCell<共享内存缓冲区>>,
        大小: (u32, u32),
    ) {
        // 尝试改变大小
        let 结果 = {
            let mut 缓冲1 = 缓冲.borrow_mut();
            缓冲1.变大小(大小)
        };

        match 结果 {
            Ok(_) => {
                // 继续使用原来的缓冲区
            }
            Err(_) => {
                // 创建新缓冲区
                let 新 = 共享内存缓冲区::new(&self.共享内存, 大小);
                let 旧 = 缓冲.replace(新);
                // 销毁之前的缓冲区
                旧.销毁();
            }
        }
    }

    // 自动双缓冲
    pub fn 取共享内存缓冲区(
        &mut self,
        大小: (u32, u32),
    ) -> Rc<RefCell<共享内存缓冲区>> {
        match &self.共享内存缓冲 {
            None => {
                // 初始化创建
                let 缓冲1 = Rc::new(RefCell::new(共享内存缓冲区::new(
                    &self.共享内存,
                    大小,
                )));
                let 缓冲2 = Rc::new(RefCell::new(共享内存缓冲区::new(
                    &self.共享内存,
                    大小,
                )));

                let 缓冲1_1 = 缓冲1.clone();

                self.共享内存缓冲 = Some(共享内存双缓冲 {
                    缓冲1,
                    缓冲2,
                    // 当前使用缓冲1
                    标志: Rc::new(RefCell::new(false)),
                });

                缓冲1_1
            }
            Some(缓冲) => {
                // 双缓冲切换
                if 缓冲.标志.borrow().clone() {
                    // 应该使用缓冲1
                    self.共享内存缓冲区检查大小(缓冲.缓冲1.clone(), 大小);
                    缓冲.标志.replace(false);

                    缓冲.缓冲1.clone()
                } else {
                    // 应该使用缓冲2
                    self.共享内存缓冲区检查大小(缓冲.缓冲2.clone(), 大小);
                    缓冲.标志.replace(true);

                    缓冲.缓冲2.clone()
                }
            }
        }
    }
}

// 使用 wl_shm (软件绘制)
//
// 像素格式: ARGB8888 (每像素 4 字节)
#[derive(Debug)]
pub struct 共享内存缓冲区 {
    池: Main<wl_shm_pool::WlShmPool>,
    缓冲区: Main<wl_buffer::WlBuffer>,

    文件: File,

    最大大小: (u32, u32),
    当前大小: (u32, u32),

    绘制标志: bool,
}

fn 共享内存创建缓冲区(
    池: &Main<wl_shm_pool::WlShmPool>,
    大小: (i32, i32),
    行间隔: i32,
) -> Main<wl_buffer::WlBuffer> {
    池.create_buffer(
        0,                        // 缓冲区在池中的开始位置
        大小.0,                 // 宽度 (像素)
        大小.1,                 // 高度 (像素)
        行间隔,                // 每行像素的字节数
        wl_shm::Format::Argb8888, // 像素格式
    )
}

impl 共享内存缓冲区 {
    // 大小: 初始大小, 最大大小
    pub fn new(共享内存: &Main<wl_shm::WlShm>, 大小: (u32, u32)) -> Self {
        let 文件 = 创建匿名文件("shm_buffer");
        // 设置文件大小
        let 文件大小 = (大小.0 * 大小.1 * 4) as i32; // 每像素 4 字节
        文件.set_len(文件大小 as u64).unwrap();

        let 池 = 共享内存.create_pool(文件.as_raw_fd(), 文件大小);

        let 缓冲区 =
            共享内存创建缓冲区(&池, (大小.0 as i32, 大小.1 as i32), (大小.0 * 4) as i32);

        Self {
            池,
            缓冲区,
            文件,
            最大大小: 大小,
            当前大小: 大小,
            绘制标志: false,
        }
    }

    // 不能超过最大大小
    pub fn 变大小(&mut self, 大小: (u32, u32)) -> Result<(), ()> {
        // 如果大小不变, 无需处理
        if (大小.0 == self.当前大小.0) && (大小.1 == self.当前大小.1) {
            return Ok(());
        }
        // 检查新大小
        if 大小.0 > self.最大大小.0 {
            return Err(());
        }
        if 大小.1 > self.最大大小.1 {
            return Err(());
        }

        // 销毁之前的缓冲区
        self.缓冲区.destroy();
        // 创建新缓冲区
        self.缓冲区 = 共享内存创建缓冲区(
            &self.池,
            (大小.0 as i32, 大小.1 as i32),
            self.行间隔字节(),
        );
        // 设置新大小
        self.当前大小 = 大小;

        Ok(())
    }

    // 每行像素间隔的字节数
    pub fn 行间隔字节(&self) -> i32 {
        (self.最大大小.0 * 4) as i32
    }

    pub fn 取绘制标志(&self) -> bool {
        self.绘制标志
    }

    pub fn 设绘制标志(&mut self, 绘制标志: bool) {
        self.绘制标志 = 绘制标志;
    }

    pub fn 取缓冲区(&self) -> &Main<wl_buffer::WlBuffer> {
        &self.缓冲区
    }

    pub fn 取文件(&mut self) -> &mut File {
        &mut self.文件
    }

    // 当前大小
    pub fn 取大小(&self) -> (u32, u32) {
        self.当前大小
    }

    pub fn 取最大大小(&self) -> (u32, u32) {
        self.最大大小
    }

    pub fn 销毁(self) {
        self.池.destroy();
        self.缓冲区.destroy();
    }
}
