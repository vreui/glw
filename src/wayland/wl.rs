//! wayland 功能封装

use std::{cell::RefCell, fs::File, os::unix::io::AsRawFd, rc::Rc};

use wayland_client::{
    protocol::{wl_buffer, wl_compositor, wl_shm, wl_surface},
    Display, EventQueue, GlobalManager, Main,
};
use wayland_protocols::xdg_shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base};

use super::input::输入管理器;
use super::util::{创建匿名文件, 填充缓冲区};

// 对 wayland 操作的封装
//
// wayland-client 0.29 API
// https://github.com/Smithay/wayland-rs/blob/v0.29.5/wayland-client/examples/simple_window.rs
//
// 注意: wayland-client 0.30 (beta) API 有重大变动 (重构)
pub struct Wl封装 {
    // wayland 服务器
    server: Display,

    事件队列: EventQueue,

    // wayland 全局服务管理器
    全局管理: GlobalManager,

    合成器: Main<wl_compositor::WlCompositor>,
    共享内存: Main<wl_shm::WlShm>,
    窗基: Main<xdg_wm_base::XdgWmBase>,

    // 保存内部状态
    窗口大小: Rc<RefCell<(f32, f32)>>,
    输入: Option<输入管理器>,
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

            窗口大小: Rc::new(RefCell::new((0.0, 0.0))),
            输入: None,
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

        // 保存窗口大小
        let 窗口大小1 = self.窗口大小.clone();

        let xdg顶级 = xdg表面.get_toplevel();
        xdg顶级.quick_assign(move |_, 事件, _| match 事件 {
            // 窗口关闭
            xdg_toplevel::Event::Close => {
                // 设置退出标志
                运行标志.replace(false);
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

                // 保存窗口大小
                窗口大小1.replace((width as f32, height as f32));
            }
            _ => unreachable!(),
        });
        xdg顶级.set_title(标题);

        let 移动窗口 = move |偏移: (f64, f64)| {
            println!("移动窗口  偏移 {:?}", 偏移);
            // TODO
        };
        let 改变大小 = move |偏移: (f64, f64), 大小: (f32, f32)| {
            println!("改变大小  补偿偏移 {:?}  窗口大小 {:?}", 偏移, 大小);
            // TODO
        };

        // 输入处理
        let 输入 = 输入管理器::new(
            &self.全局管理,
            self.共享内存.clone(),
            self.合成器.clone(),
            self.窗口大小.clone(),
            Box::new(移动窗口),
            Box::new(改变大小),
        );
        self.输入 = Some(输入);

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
