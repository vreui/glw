//! wayland 功能封装

use std::{cell::RefCell, rc::Rc};

use wayland_client::{
    protocol::{wl_buffer, wl_compositor, wl_shm, wl_surface},
    Display, EventQueue, GlobalManager, Main,
};
use wayland_protocols::xdg_shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base};

use super::buffer::缓冲区管理器;
use super::input::输入管理器;
use super::util::最小窗口大小;

// 对 wayland 操作的封装
//
// wayland-client 0.29 API
// https://github.com/Smithay/wayland-rs/blob/v0.29.5/wayland-client/examples/simple_window.rs
//
// 注意: wayland-client 0.30 (beta) API 有重大变动 (重构)
#[derive(Debug)]
pub struct Wl封装 {
    // wayland 服务器
    server: Display,

    事件队列: EventQueue,

    // wayland 全局服务管理器
    全局管理: GlobalManager,

    合成器: Main<wl_compositor::WlCompositor>,
    窗基: Main<xdg_wm_base::XdgWmBase>,

    // 保存内部状态
    窗口大小: Rc<RefCell<(f32, f32)>>,
    输入: Option<输入管理器>,

    缓冲: Rc<RefCell<缓冲区管理器>>,
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

        let 缓冲 = Rc::new(RefCell::new(缓冲区管理器::new(共享内存)));

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
            窗基,

            窗口大小: Rc::new(RefCell::new((-1.0, -1.0))),
            输入: None,

            缓冲,
        }
    }

    // wl_surface
    pub fn 创建窗口(
        &mut self,
        运行标志: Rc<RefCell<bool>>,
        标题: String,
        mut 创建缓冲区: Box<
            dyn FnMut(&mut 缓冲区管理器, (f32, f32)) -> Main<wl_buffer::WlBuffer> + 'static,
        >,
        初始大小: (f32, f32),
    ) -> (Main<wl_surface::WlSurface>, Main<xdg_toplevel::XdgToplevel>) {
        // 初始缓冲区
        let 缓冲区 = 创建缓冲区(&mut self.缓冲.borrow_mut(), 初始大小);

        let 表面 = self.合成器.create_surface();

        self.窗口大小.replace(初始大小.clone());
        // 临时窗口大小
        let 临时大小: Rc<RefCell<(f32, f32)>> =
            Rc::new(RefCell::new(self.窗口大小.borrow().clone()));

        // 用于保存窗口大小
        let 窗口大小1 = self.窗口大小.clone();
        let 临时大小1 = 临时大小.clone();
        let 表面1 = 表面.clone();
        let 缓冲1 = self.缓冲.clone();

        let xdg表面 = self.窗基.get_xdg_surface(&表面);
        xdg表面.quick_assign(move |xdg表面, 事件, _| match 事件 {
            xdg_surface::Event::Configure { serial } => {
                println!("xdg_surface (Configure)");

                // TODO 缓冲区性能优化
                // 处理改变窗口大小
                let 临时大小 = 临时大小1.borrow().clone();
                if (临时大小.0 > 0.0) && (临时大小.1 > 0.0) {
                    if 临时大小 != 窗口大小1.borrow().clone() {
                        println!("新窗口大小  {:?}", 临时大小);

                        // 重新创建缓冲区
                        let 缓冲区 = 创建缓冲区(&mut 缓冲1.borrow_mut(), 临时大小);

                        表面1.attach(Some(&缓冲区), 0, 0);
                        if 表面1.as_ref().version() >= 4 {
                            表面1.damage_buffer(0, 0, 临时大小.0 as i32, 临时大小.1 as i32);
                        } else {
                            表面1.damage(0, 0, 临时大小.0 as i32, 临时大小.1 as i32);
                        }
                        表面1.commit();
                    }

                    // 保存窗口大小
                    窗口大小1.replace(临时大小.clone());
                }
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

                临时大小.replace((width as f32, height as f32));
            }
            _ => unreachable!(),
        });
        xdg顶级.set_min_size(最小窗口大小.0, 最小窗口大小.1);
        xdg顶级.set_title(标题);

        // 输入处理
        let 输入 = 输入管理器::new(
            &self.全局管理,
            self.缓冲.borrow().取共享内存(),
            self.合成器.clone(),
            self.窗口大小.clone(),
            xdg顶级.clone(),
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
        表面.attach(Some(&缓冲区), 0, 0);
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
