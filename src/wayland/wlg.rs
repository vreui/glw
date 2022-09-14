//! wayland 全局对象管理

use std::ffi;

use wayland_client::{
    protocol::{wl_compositor, wl_seat, wl_shm},
    Display, GlobalManager, Main,
};
use wayland_protocols::xdg_shell::client::xdg_wm_base;

#[derive(Debug, Clone)]
pub struct Wl全局管理器 {
    // wayland 服务器
    server: Display,

    // wayland 全局服务管理器
    全局管理: GlobalManager,

    合成器: Main<wl_compositor::WlCompositor>,
    共享内存: Main<wl_shm::WlShm>,
    座: Main<wl_seat::WlSeat>,
    窗基: Main<xdg_wm_base::XdgWmBase>,
}

impl Wl全局管理器 {
    pub fn new(
        server: Display,
        全局管理: GlobalManager,
        合成器: Main<wl_compositor::WlCompositor>,
        共享内存: Main<wl_shm::WlShm>,
        座: Main<wl_seat::WlSeat>,
        窗基: Main<xdg_wm_base::XdgWmBase>,
    ) -> Self {
        Self {
            server,
            全局管理,
            合成器,
            共享内存,
            座,
            窗基,
        }
    }

    pub fn 取服务器(&self) -> &Display {
        &self.server
    }

    pub fn 取显示指针(&self) -> *const ffi::c_void {
        self.server.c_ptr() as *const ffi::c_void
    }

    pub fn 取全局管理(&self) -> &GlobalManager {
        &self.全局管理
    }

    pub fn 取合成器(&self) -> &Main<wl_compositor::WlCompositor> {
        &self.合成器
    }

    pub fn 取共享内存(&self) -> &Main<wl_shm::WlShm> {
        &self.共享内存
    }

    pub fn 取座(&self) -> &Main<wl_seat::WlSeat> {
        &self.座
    }

    pub fn 取窗基(&self) -> &Main<xdg_wm_base::XdgWmBase> {
        &self.窗基
    }
}
