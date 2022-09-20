//! 工具

use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

#[cfg(feature = "gleam")]
use gleam::gl;

#[cfg(feature = "egl")]
use crate::egl::Egl管理器;

#[cfg(feature = "gleam")]
pub fn 造绘制回调(
    egl: Rc<RefCell<Option<Egl管理器>>>,
    gl: Rc<RefCell<Option<Rc<dyn gl::Gl>>>>,
    背景色: Rc<RefCell<(f32, f32, f32, f32)>>,
) -> Box<dyn FnMut() -> () + 'static> {
    Box::new(move || {
        RefMut::map(egl.borrow_mut(), |a| {
            match a {
                None => {}
                Some(egl) => {
                    Ref::map(gl.borrow(), |a| {
                        match a {
                            None => {}
                            Some(gl) => {
                                窗口默认绘制(gl, 背景色.borrow().clone());

                                // 绘制结束
                                egl.交换缓冲区().unwrap();
                            }
                        }
                        a
                    });
                }
            }
            a
        });
    })
}

#[cfg(feature = "gleam")]
pub fn 窗口默认绘制(g: &Rc<dyn gl::Gl>, 颜色: (f32, f32, f32, f32)) {
    let gl = gl::ErrorCheckingGl::wrap(g.clone());

    // 清除背景
    gl.clear_color(颜色.0, 颜色.1, 颜色.2, 颜色.3);
    gl.clear(gl::COLOR_BUFFER_BIT);
}
