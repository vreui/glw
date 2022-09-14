//! xdg顶级

use std::{cell::RefCell, rc::Rc};

use wayland_client::{protocol::wl_surface, Main};
use wayland_protocols::xdg_shell::client::{xdg_surface, xdg_toplevel};

use super::t::最小窗口大小;
use super::wlg::Wl全局管理器;

#[derive(Debug, Clone)]
pub struct Xdg顶级管理器 {
    表面: Main<wl_surface::WlSurface>,

    xdg表面: Main<xdg_surface::XdgSurface>,
    顶级: Main<xdg_toplevel::XdgToplevel>,
}

impl Xdg顶级管理器 {
    // 创建 wl_surface, xdg_surface, xdg_toplevel 并初始化
    pub fn new(
        全局: &Wl全局管理器,
        // 关闭窗口回调
        mut 窗口关闭: Box<dyn FnMut() -> () + 'static>,
        // 改变大小回调 (xdg_surface::Event::Configure)
        // 不进行大小是否改变的检查, 直接回调
        // FnMut(大小: (i32, i32)) ->
        mut 改变大小: Box<dyn FnMut((i32, i32)) -> () + 'static>,
    ) -> Self {
        let 表面 = 全局.取合成器().create_surface();
        let xdg表面 = 全局.取窗基().get_xdg_surface(&表面);
        let 顶级 = xdg表面.get_toplevel();
        // 设置最小窗口大小
        顶级.set_min_size(最小窗口大小.0, 最小窗口大小.1);

        // 保存窗口大小
        let 临时大小: Rc<RefCell<(i32, i32)>> = Rc::new(RefCell::new((0, 0)));
        let 临时大小1 = 临时大小.clone();
        // 处理配置变更 (改变大小)
        xdg表面.quick_assign(move |xdg表面, 事件, _| match 事件 {
            // 确认配置生效事件
            xdg_surface::Event::Configure { serial } => {
                // DEBUG
                println!("xdg_surface (Configure)");

                // 不进行大小是否改变的检查, 直接回调
                改变大小(临时大小1.borrow().clone());

                // 确认配置
                xdg表面.ack_configure(serial);
            }
            _ => unreachable!(),
        });

        // 处理窗口关闭, 保存临时窗口大小
        顶级.quick_assign(move |_, 事件, _| match 事件 {
            xdg_toplevel::Event::Close => {
                窗口关闭();
            }
            // 临时配置 (未确认)
            xdg_toplevel::Event::Configure {
                width,
                height,
                states,
            } => {
                // DEBUG
                println!(
                    "xdg_toplevel (Configure)  ({}, {})  {:?}",
                    width, height, states
                );

                临时大小.replace((width, height));
            }
            _ => unreachable!(),
        });

        Self {
            表面,
            xdg表面,
            顶级,
        }
    }

    pub fn 取表面(&self) -> &Main<wl_surface::WlSurface> {
        &self.表面
    }

    pub fn 取xdg表面(&self) -> &Main<xdg_surface::XdgSurface> {
        &self.xdg表面
    }

    pub fn 取顶级(&self) -> &Main<xdg_toplevel::XdgToplevel> {
        &self.顶级
    }
}
