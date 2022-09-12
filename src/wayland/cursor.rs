//! 鼠标指针管理

use std::{cell::RefCell, rc::Rc};

use wayland_client::{
    protocol::{wl_compositor, wl_pointer, wl_shm, wl_surface},
    Main,
};
use wayland_cursor::{Cursor, CursorTheme};

use super::util::窗口区域;

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

fn 取指针<'a>(鼠标主题: &'a mut CursorTheme, 类型: 指针类型) -> &'a Cursor {
    // GNOME Adwaita
    match 类型 {
        指针类型::默认 => 鼠标主题.get_cursor("default").unwrap(),
        指针类型::文本 => 鼠标主题.get_cursor("text").unwrap(),
        指针类型::链接 => 鼠标主题.get_cursor("pointer").unwrap(),

        指针类型::移动 => 鼠标主题.get_cursor("move").unwrap(),
        指针类型::箭头左右 => 鼠标主题.get_cursor("ew-resize").unwrap(),
        指针类型::箭头上下 => 鼠标主题.get_cursor("ns-resize").unwrap(),
        指针类型::箭头左上右下 => 鼠标主题.get_cursor("nwse-resize").unwrap(),
        指针类型::箭头左下右上 => 鼠标主题.get_cursor("nesw-resize").unwrap(),

        指针类型::名称(n) => 鼠标主题.get_cursor(n).unwrap(),
    }
}

pub struct 指针管理器 {
    窗口大小: Rc<RefCell<(f32, f32)>>,

    // 当前鼠标指针所在的区域
    鼠标位于: Option<窗口区域>,

    鼠标: Main<wl_pointer::WlPointer>,

    // wayland_cursor
    鼠标主题: CursorTheme,

    鼠标表面: Main<wl_surface::WlSurface>,

    // wl_pointer::Event::Enter.serial
    序号: u32,
}

impl 指针管理器 {
    pub fn new(
        鼠标: Main<wl_pointer::WlPointer>,
        共享内存: Main<wl_shm::WlShm>,
        合成器: Main<wl_compositor::WlCompositor>,
        窗口大小: Rc<RefCell<(f32, f32)>>,
        指针大小: u32,
    ) -> Self {
        let mut 鼠标主题 = CursorTheme::load(指针大小, &共享内存);
        // 预加载指针图标
        let _ = 取指针(&mut 鼠标主题, 指针类型::默认);
        let _ = 取指针(&mut 鼠标主题, 指针类型::文本);
        let _ = 取指针(&mut 鼠标主题, 指针类型::链接);
        let _ = 取指针(&mut 鼠标主题, 指针类型::移动);
        let _ = 取指针(&mut 鼠标主题, 指针类型::箭头左右);
        let _ = 取指针(&mut 鼠标主题, 指针类型::箭头上下);
        let _ = 取指针(&mut 鼠标主题, 指针类型::箭头左上右下);
        let _ = 取指针(&mut 鼠标主题, 指针类型::箭头左下右上);

        let 鼠标表面 = 合成器.create_surface();

        Self {
            窗口大小,
            鼠标位于: None,
            鼠标,

            鼠标主题,

            鼠标表面,

            序号: 0,
        }
    }

    // 返回 true 表示有变化
    fn 检查鼠标区域(&mut self, 坐标: Option<(f32, f32)>) -> bool {
        let 之前 = self.鼠标位于.clone();

        match 坐标 {
            None => {
                self.鼠标位于 = None;
            }
            Some(p) => {
                let 结果 = Some(窗口区域::测试(p, self.窗口大小.borrow().clone()));
                // DEBUG
                if 结果 != 之前 {
                    println!("{:?}", 结果);
                }

                self.鼠标位于 = 结果;
            }
        }

        self.鼠标位于 != 之前
    }

    // TODO 支持动画指针
    fn 设置鼠标指针(&mut self, 指针: 指针类型) {
        let 指针 = 取指针(&mut self.鼠标主题, 指针);
        let 指针图片 = &指针[0];
        let 大小 = 指针图片.dimensions();
        let 偏移 = 指针图片.hotspot();

        // TODO 支持界面缩放 (设备像素缩放比例)
        self.鼠标表面.attach(Some(指针图片), 0, 0);

        if self.鼠标表面.as_ref().version() >= 4 {
            self.鼠标表面
                .damage_buffer(0, 0, 大小.0 as i32, 大小.1 as i32);
        } else {
            self.鼠标表面.damage(0, 0, 大小.0 as i32, 大小.1 as i32);
        }
        self.鼠标表面.commit();

        // 注意: 必须先 attach() 再 set_cursor(), 否则 GNOME (mutter) 不会显示鼠标指针
        self.鼠标.set_cursor(
            self.序号,
            Some(&self.鼠标表面),
            偏移.0 as i32,
            偏移.1 as i32,
        );
    }

    // wl_pointer::Event::Enter
    pub fn 鼠标进入(&mut self, 序号: u32, x: f64, y: f64) {
        // 首先保存进入序号
        self.序号 = 序号;

        self.检查鼠标区域(Some((x as f32, y as f32)));

        // TODO 根据区域设置不同的鼠标指针
        self.设置鼠标指针(指针类型::默认);
    }

    // wl_pointer::Event::Leave
    pub fn 鼠标离开(&mut self) {
        self.检查鼠标区域(None);
    }

    // wl_pointer::Event::Motion
    pub fn 鼠标移动(&mut self, x: f64, y: f64) {
        self.检查鼠标区域(Some((x as f32, y as f32)));

        // TODO
    }

    // wl_pointer::Event::Button
    pub fn 鼠标按键(&mut self, _按键: u32, _状态: wl_pointer::ButtonState) {
        // TODO
    }
}