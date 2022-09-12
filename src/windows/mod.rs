//! windows 平台支持

extern crate windows;

#[cfg(feature = "egl")]
extern crate glutin_egl_sys;

#[cfg(feature = "wgl")]
extern crate glutin_wgl_sys;

pub(crate) mod 接口 {
    // TODO
}

// TODO
