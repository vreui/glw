//! 鼠标指针管理

use std::{cell::RefCell, rc::Rc};

use wayland_client::{
    protocol::{wl_pointer, wl_seat, wl_surface},
    Main,
};
use wayland_cursor::{Cursor, CursorTheme};
use wayland_protocols::xdg_shell::client::xdg_toplevel;

use super::t::{指针类型, 鼠标右键, 鼠标左键};
use super::util::{窗口区域, 表面设置更新区域};
use super::wlg::Wl全局管理器;

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

#[derive(Debug)]
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

    // 用于 移动窗口/改变窗口大小
    座: Main<wl_seat::WlSeat>,
    顶级: Main<xdg_toplevel::XdgToplevel>,
    // 鼠标最后所在的坐标
    坐标: (f64, f64),
}

impl 指针管理器 {
    pub fn new(
        全局: &Wl全局管理器,
        鼠标: Main<wl_pointer::WlPointer>,
        顶级: Main<xdg_toplevel::XdgToplevel>,
        窗口大小: Rc<RefCell<(f32, f32)>>,
        指针大小: u32,
    ) -> Self {
        let mut 鼠标主题 = CursorTheme::load(指针大小, 全局.取共享内存());
        // 预加载指针图标
        let _ = 取指针(&mut 鼠标主题, 指针类型::默认);
        let _ = 取指针(&mut 鼠标主题, 指针类型::文本);
        let _ = 取指针(&mut 鼠标主题, 指针类型::链接);
        let _ = 取指针(&mut 鼠标主题, 指针类型::移动);
        let _ = 取指针(&mut 鼠标主题, 指针类型::箭头左右);
        let _ = 取指针(&mut 鼠标主题, 指针类型::箭头上下);
        let _ = 取指针(&mut 鼠标主题, 指针类型::箭头左上右下);
        let _ = 取指针(&mut 鼠标主题, 指针类型::箭头左下右上);

        let 鼠标表面 = 全局.取合成器().create_surface();

        Self {
            窗口大小,
            鼠标位于: None,
            鼠标,

            鼠标主题,

            鼠标表面,

            序号: 0,
            座: 全局.取座().clone(),
            顶级,
            坐标: (0.0, 0.0),
        }
    }

    // 返回 true 表示有变化
    fn 检查鼠标区域(&mut self, 坐标: Option<(f32, f32)>) -> bool {
        let 之前 = self.鼠标位于.clone();

        self.鼠标位于 = match 坐标 {
            None => None,
            Some(p) => Some(窗口区域::测试(p, self.窗口大小.borrow().clone())),
        };

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

        表面设置更新区域(&self.鼠标表面, (0, 0, 大小.0 as i32, 大小.1 as i32));
        self.鼠标表面.commit();

        // 注意: 必须先 attach() 再 set_cursor(), 否则 GNOME (mutter) 不会显示鼠标指针
        self.鼠标.set_cursor(
            self.序号,
            Some(&self.鼠标表面),
            偏移.0 as i32,
            偏移.1 as i32,
        );
    }

    fn 设置边框指针(&mut self) {
        let 类型 = match self.鼠标位于 {
            None => None,
            Some(a) => match a {
                窗口区域::内容 => Some(指针类型::默认),
                窗口区域::上边框 => Some(指针类型::移动),
                窗口区域::下边框 => Some(指针类型::箭头上下),
                窗口区域::左边框 | 窗口区域::右边框 => Some(指针类型::箭头左右),
                窗口区域::左上角 | 窗口区域::右下角 => Some(指针类型::箭头左上右下),
                窗口区域::右上角 | 窗口区域::左下角 => Some(指针类型::箭头左下右上),
            },
        };
        match 类型 {
            None => {}
            Some(p) => {
                self.设置鼠标指针(p);
            }
        }
    }

    // wl_pointer::Event::Enter
    pub fn 鼠标进入(&mut self, 序号: u32, x: f64, y: f64) {
        // 首先保存进入序号
        self.序号 = 序号;
        // 保存坐标
        self.坐标 = (x, y);

        self.检查鼠标区域(Some((x as f32, y as f32)));

        // 根据区域设置不同的鼠标指针
        self.设置边框指针();
    }

    // wl_pointer::Event::Leave
    pub fn 鼠标离开(&mut self) {
        self.检查鼠标区域(None);
    }

    // wl_pointer::Event::Motion
    pub fn 鼠标移动(&mut self, x: f64, y: f64) {
        // 保存坐标
        self.坐标 = (x, y);

        self.检查鼠标区域(Some((x as f32, y as f32)));

        // 根据区域设置不同的鼠标指针
        self.设置边框指针();
    }

    // wl_pointer::Event::Button
    pub fn 鼠标按键(&mut self, 按键: u32, 状态: wl_pointer::ButtonState, 序号: u32) {
        // 处理边框 (移动/改变大小 等)
        match 按键 {
            // 左键: 用于 移动窗口/改变大小
            鼠标左键 => match 状态 {
                // 按下左键
                wl_pointer::ButtonState::Pressed => match self.鼠标位于 {
                    Some(类型) => match 类型 {
                        窗口区域::内容 => {}
                        窗口区域::上边框 => {
                            // 移动
                            self.顶级._move(&self.座, 序号);
                        }
                        _ => {
                            // 改变大小
                            let 情况 = match 类型 {
                                窗口区域::下边框 => xdg_toplevel::ResizeEdge::Bottom,
                                窗口区域::左边框 => xdg_toplevel::ResizeEdge::Left,
                                窗口区域::右边框 => xdg_toplevel::ResizeEdge::Right,
                                窗口区域::左上角 => xdg_toplevel::ResizeEdge::TopLeft,
                                窗口区域::左下角 => xdg_toplevel::ResizeEdge::BottomLeft,
                                窗口区域::右上角 => xdg_toplevel::ResizeEdge::TopRight,
                                窗口区域::右下角 => xdg_toplevel::ResizeEdge::BottomRight,
                                _ => xdg_toplevel::ResizeEdge::None,
                            };
                            self.顶级.resize(&self.座, 序号, 情况);
                        }
                    },
                    _ => {}
                },
                _ => {}
            },

            // 右键: 顶部窗口菜单
            鼠标右键 => match 状态 {
                // 在顶部按下右键
                wl_pointer::ButtonState::Pressed => match self.鼠标位于 {
                    Some(类型) => match 类型 {
                        窗口区域::上边框 => {
                            // 窗口菜单
                            self.顶级.show_window_menu(
                                &self.座,
                                序号,
                                self.坐标.0 as i32,
                                self.坐标.1 as i32,
                            );
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },

            _ => {}
        }
    }
}
