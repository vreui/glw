//! 窗口封装

use windows::{
    core::{Error, HSTRING, PCWSTR},
    w,
    Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
    Win32::Graphics::Gdi::ValidateRect,
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, LoadCursorW,
        PostQuitMessage, RegisterClassExW, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, MSG,
        WINDOW_EX_STYLE, WM_DESTROY, WM_PAINT, WNDCLASSEXW, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
    },
};

pub struct 窗口封装 {
    实例: HINSTANCE,

    窗口: HWND,
}

impl 窗口封装 {
    pub unsafe fn new(标题: &str) -> Result<Self, Error> {
        let 实例 = GetModuleHandleW(None)?;
        // 实例.0 != 0

        let 窗口类名 = PCWSTR::from(w!("glw_window"));

        let 窗口类 = WNDCLASSEXW {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: 实例,
            lpszClassName: 窗口类名,

            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(glw_wndproc),
            ..Default::default()
        };
        // 注册窗口类
        let _a = RegisterClassExW(&窗口类);
        // a != 0

        let 标题 = PCWSTR::from(&HSTRING::from(标题));
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

        Ok(Self { 实例, 窗口 })
    }

    pub unsafe fn 主循环(&mut self) {
        let mut 消息 = MSG::default();

        // FIXME
        println!("GetMessageW");
        while GetMessageW(&mut 消息, HWND(0), 0, 0).into() {
            // FIXME
            println!("DispatchMessageW");
            DispatchMessageW(&消息);
        }
    }
}

// 窗口回调函数
extern "system" fn glw_wndproc(
    窗口: HWND, 消息: u32, w参数: WPARAM, l参数: LPARAM
) -> LRESULT {
    unsafe {
        // FIXME
        println!("glw_wndproc  消息 {}", 消息);
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
