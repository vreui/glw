//! 鼠标指针管理

use std::{cell::RefCell, rc::Rc};

use wayland_client::{
    protocol::{wl_compositor, wl_pointer, wl_shm, wl_surface},
    Main,
};
use wayland_cursor::{Cursor, CursorTheme};

use super::util::窗口区域;

// 鼠标按键
// wl_pointer::Event::Button.button
pub const 鼠标左键: u32 = 0x110; // 272
pub const 鼠标右键: u32 = 0x111; // 273
pub const 鼠标中键: u32 = 0x112; // 274

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

// 用于 移动窗口/改变大小
#[derive(Debug, Clone, Copy)]
enum 边框状态 {
    移动 {
        // 开始时的鼠标坐标
        起点: (f64, f64),
    },
    大小 {
        起点: (f64, f64), 类型: 窗口区域
    },
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

    // 鼠标最后出现的坐标
    坐标: (f64, f64),

    // None 表示没有 移动窗口/改变大小 (普通状态)
    状态: Option<边框状态>,

    // 回调: 移动窗口
    // FnMut(偏移: (f64, f64))
    移动窗口: Box<dyn FnMut((f64, f64)) -> () + 'static>,
    // 回调: 改变大小
    // FnMut(偏移: (f64, f64), 大小: (f32, f32))
    改变大小: Box<dyn FnMut((f64, f64), (f32, f32)) -> () + 'static>,
}

impl 指针管理器 {
    pub fn new(
        鼠标: Main<wl_pointer::WlPointer>,
        共享内存: Main<wl_shm::WlShm>,
        合成器: Main<wl_compositor::WlCompositor>,
        窗口大小: Rc<RefCell<(f32, f32)>>,
        指针大小: u32,
        移动窗口: Box<dyn FnMut((f64, f64)) -> () + 'static>,
        改变大小: Box<dyn FnMut((f64, f64), (f32, f32)) -> () + 'static>,
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
            坐标: (0.0, 0.0),
            状态: None,

            移动窗口,
            改变大小,
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
        // 保存鼠标坐标
        self.坐标 = (x, y);

        self.检查鼠标区域(Some((x as f32, y as f32)));

        // 根据区域设置不同的鼠标指针
        self.设置边框指针();
    }

    // wl_pointer::Event::Leave
    pub fn 鼠标离开(&mut self) {
        self.检查鼠标区域(None);

        self.状态 = None;
    }

    // wl_pointer::Event::Motion
    pub fn 鼠标移动(&mut self, x: f64, y: f64) {
        // 保存鼠标坐标
        self.坐标 = (x, y);

        self.检查鼠标区域(Some((x as f32, y as f32)));

        // 根据区域设置不同的鼠标指针
        self.设置边框指针();

        // 处理边框状态
        match self.状态 {
            None => {}
            Some(s) => {
                match s {
                    边框状态::移动 { 起点 } => {
                        let 偏移 = (x - 起点.0, y - 起点.1);
                        (self.移动窗口)(偏移);
                    }
                    边框状态::大小 { 起点, 类型 } => {
                        let 偏移 = (x - 起点.0, y - 起点.1);
                        // 窗口左上角应该移动的偏移
                        let mut 补偿偏移: (f64, f64) = (0.0, 0.0);
                        // 新的窗口大小
                        let mut 窗口大小 = self.窗口大小.borrow().clone();

                        match 类型 {
                            窗口区域::下边框 => {
                                窗口大小 = (窗口大小.0, 窗口大小.1 + 偏移.1 as f32);
                            }
                            窗口区域::左边框 => {
                                补偿偏移 = (偏移.0, 0.0);
                                窗口大小 = (窗口大小.0 - 偏移.0 as f32, 窗口大小.1);
                            }
                            窗口区域::右边框 => {
                                窗口大小 = (窗口大小.0 + 偏移.0 as f32, 窗口大小.1);
                            }

                            窗口区域::左上角 => {
                                补偿偏移 = (偏移.0, 偏移.1);
                                窗口大小 = (窗口大小.0 - 偏移.0 as f32, 窗口大小.1 - 偏移.1 as f32);
                            }
                            窗口区域::右下角 => {
                                窗口大小 = (窗口大小.0 + 偏移.0 as f32, 窗口大小.1 + 偏移.1 as f32);
                            }
                            窗口区域::右上角 => {
                                补偿偏移 = (0.0, 偏移.1);
                                窗口大小 = (窗口大小.0 + 偏移.0 as f32, 窗口大小.1 - 偏移.1 as f32);
                            }
                            窗口区域::左下角 => {
                                补偿偏移 = (偏移.0, 0.0);
                                窗口大小 = (窗口大小.0 - 偏移.0 as f32, 窗口大小.1 + 偏移.1 as f32);
                            }

                            _ => {}
                        }

                        (self.改变大小)(补偿偏移, 窗口大小);
                    }
                }
            }
        }
    }

    // wl_pointer::Event::Button
    pub fn 鼠标按键(&mut self, 按键: u32, 状态: wl_pointer::ButtonState) {
        // 检查边框状态, 只处理 左键
        if 按键 == 鼠标左键 {
            match 状态 {
                wl_pointer::ButtonState::Released => {
                    self.状态 = None;
                }
                wl_pointer::ButtonState::Pressed => match self.鼠标位于 {
                    None => {
                        self.状态 = None;
                    }
                    Some(类型) => match 类型 {
                        窗口区域::内容 => {
                            self.状态 = None;
                        }
                        窗口区域::上边框 => {
                            self.状态 = Some(边框状态::移动 {
                                起点: self.坐标
                            });
                        }
                        _ => {
                            self.状态 = Some(边框状态::大小 {
                                起点: self.坐标,
                                类型,
                            });
                        }
                    },
                },
                _ => {}
            }

            // DEBUG
            println!("{:?}", self.状态);
        }

        // TODO
    }
}
