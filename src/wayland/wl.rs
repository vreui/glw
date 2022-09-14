//! wayland 功能封装

use std::{cell::RefCell, rc::Rc};

use wayland_client::{
    protocol::{wl_compositor, wl_seat, wl_shm},
    Display, EventQueue, GlobalManager,
};
use wayland_protocols::xdg_shell::client::xdg_wm_base;

use super::input::输入管理器;
use super::paint::{窗口绘制管理器, 绘制参数};
use super::t::缓冲区类型;
use super::wlg::Wl全局管理器;
use super::xdgtl::Xdg顶级管理器;

// 对 wayland 操作的封装
//
// wayland-client 0.29 API
// https://github.com/Smithay/wayland-rs/blob/v0.29.5/wayland-client/examples/simple_window.rs
//
// 注意: wayland-client 0.30 (beta) API 有重大变动 (重构)
#[derive(Debug)]
pub struct Wl封装 {
    全局: Wl全局管理器,

    事件队列: EventQueue,

    // 保存内部状态
    窗口大小: Rc<RefCell<(f32, f32)>>,
    输入: Option<输入管理器>,

    绘制: Rc<RefCell<Option<窗口绘制管理器>>>,
}

impl Wl封装 {
    // 连接 wayland server 并初始化
    pub fn new() -> Self {
        // 连接 wayland
        let (server, 事件队列, 全局管理) = {
            let server = Display::connect_to_env().expect("无法连接 wayland server");

            let mut 事件队列 = server.create_event_queue();
            let 附加显示 = (*server).clone().attach(事件队列.token());
            let 全局管理 = GlobalManager::new(&附加显示);
            // 同步 wayland 服务器: 等待服务器完成处理
            事件队列
                .sync_roundtrip(&mut (), |_, _, _| unreachable!())
                .unwrap();

            (server, 事件队列, 全局管理)
        };

        // 创建 wayland 全局服务
        let 全局 = {
            let 合成器 = 全局管理
                .instantiate_exact::<wl_compositor::WlCompositor>(1)
                .unwrap();
            let 共享内存 = 全局管理.instantiate_exact::<wl_shm::WlShm>(1).unwrap();
            let 座 = 全局管理.instantiate_exact::<wl_seat::WlSeat>(1).unwrap();

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

            Wl全局管理器::new(server, 全局管理, 合成器, 共享内存, 座, 窗基)
        };

        let 绘制 = Rc::new(RefCell::new(None));

        Wl封装 {
            全局,
            事件队列,

            窗口大小: Rc::new(RefCell::new((-1.0, -1.0))),
            输入: None,

            绘制,
        }
    }

    // wl_surface
    pub fn 创建窗口(
        &mut self,
        运行标志: Rc<RefCell<bool>>,
        初始大小: (f32, f32),
        标题: String,
        绘制类型: 缓冲区类型,
        // 窗口绘制回调
        绘制回调: Box<dyn FnMut(绘制参数) -> () + 'static>,
    ) -> Xdg顶级管理器 {
        let 窗口关闭 = Box::new(move || {
            // 设置退出标志
            运行标志.replace(false);
        });

        // 处理窗口大小改变
        let 绘制1 = self.绘制.clone();
        let 窗口大小1 = self.窗口大小.clone();
        let 改变大小 = Box::new(move |大小: (i32, i32)| {
            let mut 绘制2 = 绘制1.borrow_mut();
            let 绘制 = 绘制2.as_mut().unwrap();
            绘制.改变大小(大小);

            // 更新窗口大小
            窗口大小1.replace((大小.0 as f32, 大小.1 as f32));
        });

        let 顶级 = Xdg顶级管理器::new(&self.全局, 窗口关闭, 改变大小);
        // 设置窗口标题
        顶级.取顶级().set_title(标题);

        // 输入处理
        let 输入 = 输入管理器::new(&self.全局, self.窗口大小.clone(), 顶级.取顶级().clone());
        self.输入 = Some(输入);

        // 设置绘制
        let 绘制 = 窗口绘制管理器::new(
            &self.全局,
            self.窗口大小.clone(),
            &顶级,
            绘制类型,
            绘制回调,
        );
        self.绘制.replace(Some(绘制));

        // 提交表面, 同步 wayland 服务器
        {
            顶级.取表面().commit();

            self.事件队列
                .sync_roundtrip(&mut (), |_, _, _| {
                    // 忽略
                })
                .unwrap();

            // 初始绘制
            let mut 绘制1 = self.绘制.borrow_mut();
            let 绘制 = 绘制1.as_mut().unwrap();
            绘制.初始绘制(初始大小);
        }

        顶级
    }

    // 用于主循环
    // 返回 false 表示错误
    pub fn 分发事件(&mut self) -> bool {
        self.事件队列
            .dispatch(&mut (), |_, _, _| {
                // 忽略
            })
            .is_ok()
    }

    // 释放资源, 断开 wayland 连接
    pub fn 清理(self) {
        // TODO
    }
}
