//! 窗口封装

use std::ffi;

use windows::{
    core::{Error, HSTRING, PCWSTR},
    Win32::Foundation::{GetLastError, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
    Win32::Graphics::Gdi::{GetDC, ValidateRect},
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, LoadCursorW,
        PostQuitMessage, RegisterClassExW, ShowWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT,
        IDC_ARROW, MSG, SW_SHOWNORMAL, WINDOW_EX_STYLE, WM_DESTROY, WM_PAINT, WNDCLASSEXW,
        WS_OVERLAPPEDWINDOW, WS_VISIBLE,
    },
};

#[cfg(feature = "egl")]
use super::egl::Egl实现;
#[cfg(feature = "egl")]
use super::t::GL版本;

#[derive(Debug)]
pub struct 错误(String);

impl From<Error> for 错误 {
    fn from(error: Error) -> Self {
        错误(format!("{:?}", error))
    }
}

impl From<String> for 错误 {
    fn from(文本: String) -> Self {
        错误(文本)
    }
}

pub struct 窗口封装 {
    实例: HINSTANCE,

    窗口: HWND,
}

impl 窗口封装 {
    pub unsafe fn new(标题: &str) -> Result<Self, 错误> {
        let 实例 = GetModuleHandleW(None)?;
        if 实例.0 == 0 {
            return Err(错误(format!("GetModuleHandleW()  {:?}", 实例)));
        }

        // 防止字符串内存被回收
        let 窗口类名1 = HSTRING::from("glw_window");
        let 窗口类名 = PCWSTR::from(&窗口类名1);

        let 窗口类 = WNDCLASSEXW {
            // 注意: cbSize 默认是 0 (Default::default), 需要手动设置正确
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,

            hInstance: 实例,
            lpszClassName: 窗口类名,
            lpfnWndProc: Some(glw_wndproc),

            style: CS_HREDRAW | CS_VREDRAW,
            hCursor: LoadCursorW(None, IDC_ARROW)?,

            ..Default::default()
        };

        // 注册窗口类
        let a = RegisterClassExW(&窗口类);
        if a == 0 {
            return Err(错误(format!("RegisterClassExW()  {:?}", a)));
        }

        // 防止字符串内存被回收
        let 标题1 = HSTRING::from(标题);
        let 标题 = PCWSTR::from(&标题1);
        // 创建窗口
        let 窗口 = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            窗口类名,
            标题,
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            实例,
            std::ptr::null(),
        );
        if 窗口.0 == 0 {
            let 错误码 = GetLastError();
            return Err(错误(format!(
                "CreateWindowExW()  {:?}  [{:?}]",
                窗口, 错误码
            )));
        }

        Ok(Self { 实例, 窗口 })
    }

    #[cfg(feature = "egl")]
    pub unsafe fn 初始化gl(&mut self) -> Result<Egl实现, String> {
        let 显示指针 = GetDC(self.窗口).0 as *const ffi::c_void;
        let 窗口指针 = self.窗口.0 as *const ffi::c_void;

        Egl实现::new(GL版本, 显示指针, 窗口指针)
    }

    pub unsafe fn 主循环(&mut self) {
        // 显示窗口
        ShowWindow(self.窗口, SW_SHOWNORMAL);

        let mut 消息 = MSG::default();

        while GetMessageW(&mut 消息, HWND(0), 0, 0).into() {
            DispatchMessageW(&消息);
        }
    }
}

// 窗口回调函数
extern "system" fn glw_wndproc(
    窗口: HWND, 消息: u32, w参数: WPARAM, l参数: LPARAM
) -> LRESULT {
    unsafe {
        match 消息 {
            WM_PAINT => {
                println!("WM_PAINT");
                ValidateRect(窗口, None);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(窗口, 消息, w参数, l参数),
        }
    }
}
