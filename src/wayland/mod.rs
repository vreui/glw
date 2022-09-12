//! wayland (GNU/Linux) 支持

extern crate libc;
extern crate nix;
extern crate wayland_client;
extern crate wayland_cursor;
extern crate wayland_protocols;

#[cfg(feature = "egl")]
extern crate wayland_egl;

// TODO
//extern crate wayland_sys;

#[cfg(feature = "egl")]
extern crate glutin_egl_sys;

use std::{
    cell::RefCell,
    ffi::CString,
    fs::File,
    io::{BufWriter, Write},
    os::unix::io::{AsRawFd, FromRawFd},
    rc::Rc,
};

use nix::{
    sys::memfd::{memfd_create, MemFdCreateFlag},
    unistd::{lseek, Whence},
};

use wayland_client::{
    event_enum,
    protocol::{wl_buffer, wl_compositor, wl_keyboard, wl_pointer, wl_seat, wl_shm, wl_surface},
    Display, EventQueue, Filter, GlobalManager, Main,
};
use wayland_protocols::xdg_shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base};

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
fn 填充缓冲区(文件: &mut File, 大小: (u32, u32), 颜色: (f32, f32, f32, f32)) {
    // 从头开始写文件
    lseek(文件.as_raw_fd(), 0, Whence::SeekSet).unwrap();

    // ARGB
    let 最大 = 0xff as f32;
    let 像素 = ((((最大 * 颜色.3) as u32) << 24)
        | (((最大 * 颜色.0) as u32) << 16)
        | (((最大 * 颜色.1) as u32) << 8)
        | ((最大 * 颜色.2) as u32))
        .to_ne_bytes();

    let mut 写 = BufWriter::new(文件);
    // 填充像素
    for _y in 0..大小.1 {
        for _x in 0..大小.0 {
            写.write_all(&像素).unwrap();
        }
    }

    写.flush().unwrap();
}

event_enum!(
    Events |
    Pointer => wl_pointer::WlPointer,
    Keyboard => wl_keyboard::WlKeyboard
);

// 对 wayland 操作的封装
//
// wayland-client 0.29 API
// https://github.com/Smithay/wayland-rs/blob/v0.29.5/wayland-client/examples/simple_window.rs
//
// 注意: wayland-client 0.30 (beta) API 有重大变动 (重构)
struct Wl封装 {
    // wayland 服务器
    server: Display,

    事件队列: EventQueue,

    // wayland 全局服务管理器
    全局管理: GlobalManager,

    合成器: Main<wl_compositor::WlCompositor>,
    共享内存: Main<wl_shm::WlShm>,
    窗基: Main<xdg_wm_base::XdgWmBase>,
}

impl Wl封装 {
    // 连接 wayland server 并初始化
    pub fn new() -> Self {
        let server = Display::connect_to_env().expect("无法连接 wayland server");

        let mut 事件队列 = server.create_event_queue();
        let 附加显示 = (*server).clone().attach(事件队列.token());
        let 全局管理 = GlobalManager::new(&附加显示);
        // 同步 wayland 服务器: 等待服务器完成处理
        事件队列
            .sync_roundtrip(&mut (), |_, _, _| unreachable!())
            .unwrap();

        let 合成器 = 全局管理
            .instantiate_exact::<wl_compositor::WlCompositor>(1)
            .unwrap();
        let 共享内存 = 全局管理.instantiate_exact::<wl_shm::WlShm>(1).unwrap();

        let 窗基 = 全局管理
            .instantiate_exact::<xdg_wm_base::XdgWmBase>(2)
            .expect("不支持 xdg_shell");
        // 窗口 ping-pong 保活消息:
        // 未及时响应服务器发来的 ping 消息可能会被杀掉窗口
        窗基.quick_assign(|窗基, 事件, _| match 事件 {
            xdg_wm_base::Event::Ping { serial } => {
                窗基.pong(serial);
            }
            _ => {}
        });

        Wl封装 {
            server,
            事件队列,
            全局管理,
            合成器,
            共享内存,
            窗基,
        }
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

        (缓冲区, 文件)
    }

    // EGL
    pub fn 创建egl缓冲区(&mut self) {
        // TODO
    }

    // wl_surface
    pub fn 创建窗口(
        &mut self,
        运行标志: Rc<RefCell<bool>>,
        标题: String,
        缓冲区: &Main<wl_buffer::WlBuffer>,
    ) -> (Main<wl_surface::WlSurface>, Main<xdg_toplevel::XdgToplevel>) {
        let 表面 = self.合成器.create_surface();

        let xdg表面 = self.窗基.get_xdg_surface(&表面);
        xdg表面.quick_assign(move |xdg表面, 事件, _| match 事件 {
            xdg_surface::Event::Configure { serial } => {
                println!("xdg_surface (Configure)");
                xdg表面.ack_configure(serial);
            }
            _ => unreachable!(),
        });

        let xdg顶级 = xdg表面.get_toplevel();
        xdg顶级.quick_assign(move |_, 事件, _| match 事件 {
            // 窗口关闭
            xdg_toplevel::Event::Close => {
                // 设置退出标志
                运行标志.replace(false);

                println!("关闭窗口");
            }
            xdg_toplevel::Event::Configure {
                width,
                height,
                states,
            } => {
                println!(
                    "xdg_toplevel (Configure)  {} x {}  ({:?})",
                    width, height, states
                );
            }
            _ => unreachable!(),
        });
        xdg顶级.set_title(标题);

        // 事件处理器
        let 过滤器 = Filter::new(move |事件, _, _| match 事件 {
            // 鼠标事件
            Events::Pointer { event, .. } => match event {
                wl_pointer::Event::Enter {
                    surface_x,
                    surface_y,
                    ..
                } => {
                    println!("鼠标进入  ({}, {})", surface_x, surface_y);
                }
                wl_pointer::Event::Leave { .. } => {
                    println!("鼠标离开");
                }
                wl_pointer::Event::Motion {
                    surface_x,
                    surface_y,
                    ..
                } => {
                    println!("鼠标移动  ({}, {})", surface_x, surface_y);
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

        // TODO 处理键盘/鼠标动态添加移除
        let mut 鼠标已创建 = false;
        let mut 键盘已创建 = false;
        self.全局管理
            .instantiate_exact::<wl_seat::WlSeat>(1)
            .unwrap()
            .quick_assign(move |座, 事件, _| match 事件 {
                wl_seat::Event::Capabilities { capabilities } => {
                    // 鼠标和键盘只创建一次
                    if !鼠标已创建 && capabilities.contains(wl_seat::Capability::Pointer) {
                        座.get_pointer().assign(过滤器.clone());
                        鼠标已创建 = true;
                    }
                    if !键盘已创建 && capabilities.contains(wl_seat::Capability::Keyboard) {
                        座.get_keyboard().assign(过滤器.clone());
                        键盘已创建 = true;
                    }
                }
                _ => {}
            });

        // 提交表面, 同步 wayland 服务器
        表面.commit();

        self.事件队列
            .sync_roundtrip(&mut (), |_, _, _| {
                // 忽略
            })
            .unwrap();

        // 附加缓冲区
        表面.attach(Some(缓冲区), 0, 0);
        表面.commit();

        (表面, xdg顶级)
    }

    // 用于主循环
    // 返回 false 表示错误
    pub fn 分发事件(&mut self) -> bool {
        // TODO
        self.事件队列
            .dispatch(&mut (), |_, _, _| {
                // TODO
            })
            .is_ok()
    }

    // 释放资源, 断开 wayland 连接
    pub fn 清理(self) {
        // TODO
    }
}

pub(crate) mod 接口 {
    use std::{cell::RefCell, fs::File, rc::Rc};

    use wayland_client::{
        protocol::{wl_buffer, wl_surface},
        Main,
    };
    use wayland_protocols::xdg_shell::client::xdg_toplevel;

    use super::Wl封装;
    use crate::api::{内部窗口接口, 窗口创建参数};

    // TODO 多窗口支持
    pub struct 内部窗口 {
        #[allow(dead_code)]
        非线程安全: Rc<()>,

        标题: String,
        背景色: (f32, f32, f32, f32),

        wl: Wl封装,
        运行标志: Rc<RefCell<bool>>,

        缓冲区: Main<wl_buffer::WlBuffer>,
        // 共享内存缓冲区
        缓冲区文件: File,

        表面: Main<wl_surface::WlSurface>,
        xdg顶级: Main<xdg_toplevel::XdgToplevel>,
    }

    impl 内部窗口 {
        pub fn new(参数: 窗口创建参数) -> Self {
            let mut wl = Wl封装::new();

            // 创建缓冲区
            let (缓冲区, 缓冲区文件) = wl
                .创建共享内存缓冲区((参数.大小.0 as u32, 参数.大小.1 as u32), 参数.背景色);
            // TODO 支持 EGL 缓冲区

            let 运行标志 = Rc::new(RefCell::new(false));
            // 创建窗口
            let (表面, xdg顶级) = wl.创建窗口(运行标志.clone(), 参数.标题.to_string(), &缓冲区);

            Self {
                非线程安全: Rc::new(()),

                标题: 参数.标题.to_string(),
                背景色: 参数.背景色,

                wl,
                运行标志,

                缓冲区,
                缓冲区文件,
                表面,
                xdg顶级,
            }
        }
    }

    //impl !Send for 内部窗口 {}
    //impl !Sync for 内部窗口 {}

    impl 内部窗口接口 for 内部窗口 {
        fn 取标题(&self) -> &str {
            // TODO
            &self.标题
        }

        fn 设标题(&mut self, 标题: &str) {
            // TODO
            self.标题 = 标题.to_string();
        }

        fn 取大小(&self) -> (i32, i32) {
            // TODO
            (0, 0)
        }

        fn 设大小(&mut self, _大小: (i32, i32)) {
            // TODO
        }

        fn 取背景色(&self) -> (f32, f32, f32, f32) {
            self.背景色
        }

        fn 设背景色(&mut self, 背景色: (f32, f32, f32, f32)) {
            self.背景色 = 背景色;
        }

        fn 主循环(&mut self) {
            self.运行标志.replace(true);

            while self.运行标志.borrow().clone() {
                if !self.wl.分发事件() {
                    break;
                }
            }

            self.运行标志.replace(false);
        }

        fn 清理(self) {
            self.wl.清理();
        }
    }
}
