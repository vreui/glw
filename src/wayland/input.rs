//! 输入处理

use std::{cell::RefCell, rc::Rc};

use wayland_client::{
    event_enum,
    protocol::{wl_keyboard, wl_pointer, wl_seat},
    Filter, Main,
};
use wayland_protocols::xdg_shell::client::xdg_toplevel;

use super::cursor::指针管理器;
use super::wlg::Wl全局管理器;

event_enum!(
  Events |
  Pointer => wl_pointer::WlPointer,
  Keyboard => wl_keyboard::WlKeyboard
);

#[derive(Debug)]
pub struct 输入管理器 {
    // 鼠标
    指针: Rc<RefCell<Option<指针管理器>>>,

    键盘: Rc<RefCell<Option<Main<wl_keyboard::WlKeyboard>>>>,
}

impl 输入管理器 {
    pub fn new(
        全局: &Wl全局管理器,
        窗口大小: Rc<RefCell<(f32, f32)>>,
        顶级: Main<xdg_toplevel::XdgToplevel>,
    ) -> Self {
        let 指针: Rc<RefCell<Option<指针管理器>>> = Rc::new(RefCell::new(None));
        let 键盘: Rc<RefCell<Option<Main<wl_keyboard::WlKeyboard>>>> = Rc::new(RefCell::new(None));

        // 事件处理
        let 指针1 = 指针.clone();
        let 过滤器 = Filter::new(move |事件, _, _| match 事件 {
            // 鼠标事件
            Events::Pointer { event, .. } => match event {
                wl_pointer::Event::Enter {
                    serial,
                    surface_x,
                    surface_y,
                    ..
                } => {
                    println!("鼠标进入  ({}, {})", surface_x, surface_y);

                    let mut 指针2 = 指针1.borrow_mut();
                    let 指针 = 指针2.as_mut().unwrap();
                    指针.鼠标进入(serial, surface_x, surface_y);
                }
                wl_pointer::Event::Leave { .. } => {
                    println!("鼠标离开");

                    let mut 指针2 = 指针1.borrow_mut();
                    let 指针 = 指针2.as_mut().unwrap();
                    指针.鼠标离开();
                }
                wl_pointer::Event::Motion {
                    surface_x,
                    surface_y,
                    ..
                } => {
                    let mut 指针2 = 指针1.borrow_mut();
                    let 指针 = 指针2.as_mut().unwrap();
                    指针.鼠标移动(surface_x, surface_y);
                }
                wl_pointer::Event::Button {
                    button,
                    state,
                    serial,
                    ..
                } => {
                    println!("鼠标按键  {}  ({:?})", button, state);

                    let mut 指针2 = 指针1.borrow_mut();
                    let 指针 = 指针2.as_mut().unwrap();
                    指针.鼠标按键(button, state, serial);
                }
                _ => {}
            },
            // 键盘事件
            Events::Keyboard { event, .. } => match event {
                wl_keyboard::Event::Enter { .. } => {
                    println!("键盘获得焦点");
                }
                wl_keyboard::Event::Leave { .. } => {
                    println!("键盘失去焦点");
                }
                wl_keyboard::Event::Key { key, state, .. } => {
                    println!("键盘按键  {}  ({:?})", key, state);
                }
                _ => {}
            },
        });

        // TODO 处理 鼠标/键盘 动态添加移除
        let 指针1 = 指针.clone();
        let 键盘1 = 键盘.clone();
        let 窗口大小1 = 窗口大小.clone();
        let 全局1 = 全局.clone();

        let 座 = 全局.取座();
        座.quick_assign(move |座, 事件, _| match 事件 {
            wl_seat::Event::Capabilities { capabilities } => {
                // 鼠标和键盘只创建一次
                if 指针1.borrow().is_none() && capabilities.contains(wl_seat::Capability::Pointer)
                {
                    let 鼠标 = 座.get_pointer();
                    鼠标.assign(过滤器.clone());

                    let 指针 =
                        指针管理器::new(&全局1, 鼠标, 顶级.clone(), 窗口大小1.clone(), 32);
                    指针1.replace(Some(指针));
                }
                if 键盘1.borrow().is_none() && capabilities.contains(wl_seat::Capability::Keyboard)
                {
                    let 键盘 = 座.get_keyboard();
                    键盘.assign(过滤器.clone());
                    键盘1.replace(Some(键盘));
                }
            }
            _ => {}
        });

        Self { 指针, 键盘 }
    }
}
