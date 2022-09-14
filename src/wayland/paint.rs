//! 窗口绘制管理器

use std::fmt::{Debug, Formatter};
use std::{cell::RefCell, fs::File, rc::Rc};

use super::buffer::{共享内存缓冲区, 缓冲区管理器};
use super::t::缓冲区类型;
use super::util::表面设置更新区域;
use super::wlg::Wl全局管理器;
use super::xdgtl::Xdg顶级管理器;

// 绘制窗口用的参数
#[derive(Debug)]
pub struct 绘制参数<'a> {
    // 当前分辨率
    pub 大小: (u32, u32),
    pub 最大大小: (u32, u32),

    // 是否首次绘制标志
    pub 已绘制: bool,

    // TODO 支持 EGL

    // 每行像素间隔的字节数
    pub 行间隔: i32,

    // 共享内存缓冲区对应的文件
    pub 文件: &'a mut File,
}

// TODO 支持 EGL
//#[derive(Debug)]
pub struct 窗口绘制管理器 {
    窗口大小: Rc<RefCell<(f32, f32)>>,
    缓冲: 缓冲区管理器,

    顶级: Xdg顶级管理器,
    绘制类型: 缓冲区类型,
    绘制回调: Box<dyn FnMut(绘制参数) -> () + 'static>,
}

impl Debug for 窗口绘制管理器 {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // TODO
        Ok(())
    }
}

impl 窗口绘制管理器 {
    pub fn new(
        全局: &Wl全局管理器,
        窗口大小: Rc<RefCell<(f32, f32)>>,
        顶级: &Xdg顶级管理器,
        绘制类型: 缓冲区类型,
        绘制回调: Box<dyn FnMut(绘制参数) -> () + 'static>,
    ) -> Self {
        let 缓冲 = 缓冲区管理器::new(全局.取共享内存().clone());

        Self {
            窗口大小,
            缓冲,

            顶级: 顶级.clone(),
            绘制类型,
            绘制回调,
        }
    }

    // TODO 支持 EGL
    pub fn 初始绘制(&mut self, 初始大小: (f32, f32)) {
        let 大小 = (初始大小.0 as u32, 初始大小.1 as u32);
        let 缓冲1 = self.缓冲.取共享内存缓冲区(大小);
        let mut 缓冲 = 缓冲1.borrow_mut();

        (self.绘制回调)(绘制参数 {
            大小,
            最大大小: 缓冲.取最大大小(),
            已绘制: 缓冲.取绘制标志(),
            行间隔: 缓冲.行间隔字节(),
            文件: 缓冲.取文件(),
        });
        缓冲.设绘制标志(true);

        let 表面 = self.顶级.取表面();
        // 附加缓冲区
        表面.attach(Some(缓冲.取缓冲区()), 0, 0);
        表面.commit();
    }

    // 处理窗口大小改变 (绘制)
    pub fn 改变大小(&mut self, 大小: (i32, i32)) {
        // 检查大小
        if (大小.0 <= 0) || (大小.1 <= 0) {
            return;
        }
        // 检查大小是否改变
        let 窗口大小 = self.窗口大小.borrow().clone();
        if (窗口大小.0 as i32 == 大小.0) && (窗口大小.1 as i32 == 大小.1) {
            return;
        }

        // 重新绘制
        let 大小 = (大小.0 as u32, 大小.1 as u32);
        let 缓冲1 = self.缓冲.取共享内存缓冲区(大小);
        let mut 缓冲 = 缓冲1.borrow_mut();

        (self.绘制回调)(绘制参数 {
            大小,
            最大大小: 缓冲.取最大大小(),
            已绘制: 缓冲.取绘制标志(),
            行间隔: 缓冲.行间隔字节(),
            文件: 缓冲.取文件(),
        });
        缓冲.设绘制标志(true);

        let 表面 = self.顶级.取表面();
        // 附加缓冲区
        表面.attach(Some(缓冲.取缓冲区()), 0, 0);
        表面设置更新区域(表面, (0, 0, 大小.0 as i32, 大小.1 as i32));
        表面.commit();
    }
}
