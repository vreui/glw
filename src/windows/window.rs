//! 窗口封装

use std::{
    cell::{RefCell, RefMut},
    ffi,
    rc::Rc,
};

use windows::{
    core::{Error, HSTRING, PCWSTR},
    Win32::Foundation::{GetLastError, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
    Win32::Graphics::Gdi::{GetDC, RedrawWindow, ValidateRect, RDW_INVALIDATE},
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, GetWindowLongPtrW,
        KillTimer, LoadCursorW, PostQuitMessage, RegisterClassExW, SetTimer, SetWindowLongPtrW,
        ShowWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, MSG, SW_SHOWNORMAL,
        WINDOW_EX_STYLE, WINDOW_LONG_PTR_INDEX, WM_DESTROY, WM_LBUTTONDOWN, WM_PAINT, WM_SIZE,
        WM_TIMER, WNDCLASSEXW, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
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

struct 窗口数据 {
    pub 绘制回调: Rc<RefCell<Option<Box<dyn FnMut() -> () + 'static>>>>,
}

pub struct 窗口封装 {
    实例: HINSTANCE,

    窗口: HWND,

    // 绘制回调函数
    绘制回调: Rc<RefCell<Option<Box<dyn FnMut() -> () + 'static>>>>,

    // 用于窗口函数访问
    数据: Box<窗口数据>,
}

impl 窗口封装 {
    pub unsafe fn new(大小: (i32, i32), 标题: &str) -> Result<Self, 错误> {
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

            // 每个窗口实例附带的数据字节数 (用于 SetWindowLongPtrW)
            cbWndExtra: std::mem::size_of::<*const ffi::c_void>() as i32,

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
            大小.0,
            大小.1,
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

        let 绘制回调: Rc<RefCell<Option<Box<dyn FnMut() -> () + 'static>>>> =
            Rc::new(RefCell::new(None));

        // 窗口数据
        let 数据 = Box::new(窗口数据 {
            绘制回调: 绘制回调.clone(),
        });
        // 设置数据指针
        let 窗口数据指针: *const _ = &*数据;
        SetWindowLongPtrW(窗口, WINDOW_LONG_PTR_INDEX(0), 窗口数据指针 as isize);

        Ok(Self {
            实例,
            窗口,
            绘制回调,
            数据,
        })
    }

    pub fn 设绘制回调(&mut self, 回调: Option<Box<dyn FnMut() -> () + 'static>>) {
        self.绘制回调.replace(回调);
    }

    // 请求重绘窗口
    pub fn 请求绘制(&mut self) {
        unsafe {
            RedrawWindow(self.窗口, None, None, RDW_INVALIDATE);
        }
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

const fn loword(x: u32) -> u16 {
    (x & 0xFFFF) as u16
}

const fn hiword(x: u32) -> u16 {
    ((x >> 16) & 0xFFFF) as u16
}

// 窗口回调函数
extern "system" fn glw_wndproc(
    窗口: HWND, 消息: u32, w参数: WPARAM, l参数: LPARAM
) -> LRESULT {
    unsafe fn 取窗口数据(窗口: HWND) -> *const 窗口数据 {
        let 指针 = GetWindowLongPtrW(窗口, WINDOW_LONG_PTR_INDEX(0));
        指针 as *const 窗口数据
    }

    const 重绘定时器: usize = 1;

    unsafe {
        match 消息 {
            WM_SIZE => {
                // 获取新的窗口大小
                let 大小 = (loword(l参数.0 as u32), hiword(l参数.0 as u32));
                // DEBUG
                println!("WM_SIZE {:?}", 大小);

                // 延迟重绘窗口 10ms
                SetTimer(窗口, 重绘定时器, 10, None);

                RedrawWindow(窗口, None, None, RDW_INVALIDATE);
                LRESULT(0)
            }
            WM_TIMER => {
                // 延迟重绘窗口: 用于修复改变大小的渲染 BUG
                if w参数.0 == 重绘定时器 {
                    RedrawWindow(窗口, None, None, RDW_INVALIDATE);

                    // 只重绘一次
                    KillTimer(窗口, 重绘定时器);
                }

                LRESULT(0)
            }

            WM_PAINT => {
                // DEBUG
                println!("WM_PAINT");

                // 绘制回调
                let 数据 = 取窗口数据(窗口);
                RefMut::map((*数据).绘制回调.borrow_mut(), |a| {
                    match a {
                        None => {}
                        Some(回调) => {
                            (回调)();
                        }
                    }
                    a
                });

                ValidateRect(窗口, None);
                LRESULT(0)
            }

            // 鼠标键盘消息
            WM_LBUTTONDOWN => {
                println!("WM_LBUTTONDOWN");
                // DEBUG
                RedrawWindow(窗口, None, None, RDW_INVALIDATE);

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
