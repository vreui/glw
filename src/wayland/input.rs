//! 输入处理

use std::{cell::RefCell, rc::Rc};

use wayland_client::{
    event_enum,
    protocol::{wl_buffer, wl_compositor, wl_keyboard, wl_pointer, wl_seat, wl_shm, wl_surface},
    Filter, GlobalManager, Main,
};
use wayland_cursor::{Cursor, CursorTheme};

use super::util::窗口区域;

event_enum!(
  Events |
  Pointer => wl_pointer::WlPointer,
  Keyboard => wl_keyboard::WlKeyboard
);

pub struct 输入管理器 {
    窗口大小: Rc<RefCell<(f32, f32)>>,

    // 当前鼠标指针所在的区域
    鼠标位于: Rc<RefCell<Option<窗口区域>>>,

    座: Main<wl_seat::WlSeat>,

    鼠标: Rc<RefCell<Option<Main<wl_pointer::WlPointer>>>>,
    键盘: Rc<RefCell<Option<Main<wl_keyboard::WlKeyboard>>>>,
}

impl 输入管理器 {
    pub fn new(
        全局管理: &GlobalManager,
        共享内存: &Main<wl_shm::WlShm>,
        合成器: &Main<wl_compositor::WlCompositor>,
        窗口大小: Rc<RefCell<(f32, f32)>>,
    ) -> Self {
        // 鼠标指针
        let mut 鼠标主题 = CursorTheme::load(32, 共享内存);
        // 预加载 (GNOME Adwaita)
        let _指针默认 = 鼠标主题.get_cursor("default").unwrap();
        let _指针文本 = 鼠标主题.get_cursor("text").unwrap();
        let _指针链接 = 鼠标主题.get_cursor("pointer").unwrap();
        let _指针移动 = 鼠标主题.get_cursor("move").unwrap();
        let _指针箭头左右 = 鼠标主题.get_cursor("ew-resize").unwrap();
        let _指针箭头上下 = 鼠标主题.get_cursor("ns-resize").unwrap();
        let _指针箭头左上右下 = 鼠标主题.get_cursor("nwse-resize").unwrap();
        let _指针箭头左下右上 = 鼠标主题.get_cursor("nesw-resize").unwrap();

        let 鼠标表面 = 合成器.create_surface();

        // TODO
        鼠标表面.commit();

        // wl_seat
        let 座 = 全局管理.instantiate_exact::<wl_seat::WlSeat>(1).unwrap();
        let 鼠标: Rc<RefCell<Option<Main<wl_pointer::WlPointer>>>> = Rc::new(RefCell::new(None));
        let 键盘: Rc<RefCell<Option<Main<wl_keyboard::WlKeyboard>>>> = Rc::new(RefCell::new(None));

        let 鼠标位于 = Rc::new(RefCell::new(None));

        let 鼠标位于1 = 鼠标位于.clone();
        let 窗口大小1 = 窗口大小.clone();
        let 检查鼠标区域 = move |坐标: Option<(f64, f64)>| {
            let 之前 = 鼠标位于1.borrow().clone();

            match 坐标 {
                None => {
                    鼠标位于1.replace(None);
                }
                Some(p) => {
                    let 结果 = Some(窗口区域::测试(
                        (p.0 as f32, p.1 as f32),
                        窗口大小1.borrow().clone(),
                    ));

                    // DEBUG
                    if 结果 != 之前 {
                        println!("{:?}", 结果);
                    }

                    鼠标位于1.replace(结果);
                }
            }
        };

        // DEBUG 自绘鼠标
        let (自绘鼠标, 自绘鼠标文件) = {
            use super::util::{创建匿名文件, 填充缓冲区};
            use std::os::unix::io::AsRawFd;

            let mut 文件 = 创建匿名文件("cursor shm_buffer");
            填充缓冲区(&mut 文件, (32, 32), (0.5, 0.5, 0.5, 0.5));

            let 池 = 共享内存.create_pool(文件.as_raw_fd(), (32 * 32 * 4));
            let 缓冲区 = 池.create_buffer(0, 32, 32, (32 * 4), wl_shm::Format::Argb8888);

            (缓冲区, 文件)
        };

        // 事件处理
        let 鼠标1 = 鼠标.clone();
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

                    检查鼠标区域(Some((surface_x, surface_y)));

                    // TODO 根据区域设置不同的鼠标指针
                    // 设置鼠标指针
                    鼠标1
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .set_cursor(serial, Some(&鼠标表面), 0, 0);

                    let 指针默认 = 鼠标主题.get_cursor("default").unwrap();
                    let 指针图片 = &指针默认[0];
                    let 偏移 = 指针图片.hotspot();
                    // DEBUG
                    println!("指针偏移 {:?}", 偏移);

                    鼠标表面.attach(Some(&指针默认[0]), -(偏移.0 as i32), -(偏移.1 as i32));
                    //鼠标表面.attach(Some(&自绘鼠标), 0, 0);
                    鼠标表面.commit();
                }
                wl_pointer::Event::Leave { .. } => {
                    println!("鼠标离开");

                    检查鼠标区域(None);
                }
                wl_pointer::Event::Motion {
                    surface_x,
                    surface_y,
                    ..
                } => {
                    println!("鼠标移动  ({}, {})", surface_x, surface_y);

                    检查鼠标区域(Some((surface_x, surface_y)));
                }
                wl_pointer::Event::Button { button, state, .. } => {
                    println!("鼠标按键  {}  ({:?})", button, state);
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
        let 鼠标1 = 鼠标.clone();
        let 键盘1 = 键盘.clone();
        座.quick_assign(move |座, 事件, _| match 事件 {
            wl_seat::Event::Capabilities { capabilities } => {
                // 鼠标和键盘只创建一次
                if 鼠标1.borrow().is_none() && capabilities.contains(wl_seat::Capability::Pointer)
                {
                    let 鼠标 = 座.get_pointer();
                    鼠标.assign(过滤器.clone());
                    鼠标1.replace(Some(鼠标));
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
            窗口大小,
            鼠标位于,
            座,
            鼠标,
            键盘,
        }
    }
}
