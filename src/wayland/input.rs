//! 输入处理

use std::{cell::RefCell, rc::Rc};

use wayland_client::{
    event_enum,
    protocol::{wl_compositor, wl_keyboard, wl_pointer, wl_seat, wl_shm},
    Filter, GlobalManager, Main,
};

use super::cursor::指针管理器;

event_enum!(
  Events |
  Pointer => wl_pointer::WlPointer,
  Keyboard => wl_keyboard::WlKeyboard
);

pub struct 输入管理器 {
    座: Main<wl_seat::WlSeat>,

    键盘: Rc<RefCell<Option<Main<wl_keyboard::WlKeyboard>>>>,

    // 鼠标
    指针: Rc<RefCell<Option<指针管理器>>>,
}

impl 输入管理器 {
    pub fn new(
        全局管理: &GlobalManager,
        共享内存: Main<wl_shm::WlShm>,
        合成器: Main<wl_compositor::WlCompositor>,
        窗口大小: Rc<RefCell<(f32, f32)>>,
        移动窗口: Box<dyn FnMut((f64, f64)) -> () + 'static>,
        改变大小: Box<dyn FnMut((f64, f64), (f32, f32)) -> () + 'static>,
    ) -> Self {
        let 指针: Rc<RefCell<Option<指针管理器>>> = Rc::new(RefCell::new(None));

        // wl_seat
        let 座 = 全局管理.instantiate_exact::<wl_seat::WlSeat>(1).unwrap();
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
                wl_pointer::Event::Button { button, state, .. } => {
                    println!("鼠标按键  {}  ({:?})", button, state);

                    let mut 指针2 = 指针1.borrow_mut();
                    let 指针 = 指针2.as_mut().unwrap();
                    指针.鼠标按键(button, state);
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
        let mut 移动窗口 = Some(移动窗口);
        let mut 改变大小 = Some(改变大小);
        座.quick_assign(move |座, 事件, _| match 事件 {
            wl_seat::Event::Capabilities { capabilities } => {
                // 鼠标和键盘只创建一次
                if 指针1.borrow().is_none() && capabilities.contains(wl_seat::Capability::Pointer)
                {
                    let 鼠标 = 座.get_pointer();
                    鼠标.assign(过滤器.clone());

                    let 指针 = 指针管理器::new(
                        鼠标,
                        共享内存.clone(),
                        合成器.clone(),
                        窗口大小1.clone(),
                        32,
                        移动窗口.take().unwrap(),
                        改变大小.take().unwrap(),
                    );
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

        Self {
            座, 键盘, 指针
        }
    }
}
