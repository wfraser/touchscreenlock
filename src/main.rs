#![windows_subsystem = "windows"]
#![allow(non_snake_case)]

use std::mem::size_of;
use std::ptr::null_mut;
use utf16_lit::utf16_null;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, PWSTR, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, MONITORINFO, MONITOR_DEFAULTTONULL, MonitorFromWindow, UpdateWindow};
use windows::Win32::System::Threading::{GetStartupInfoW, STARTF_USESHOWWINDOW, STARTUPINFOW};
use windows::Win32::UI::WindowsAndMessaging::{CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, HWND_TOP, IDC_CROSS, LoadCursorW, LoadIconW, MSG, PostQuitMessage, RegisterClassExW, SHOW_WINDOW_CMD, SW_SHOWNORMAL, SetWindowPos, ShowWindow, TranslateMessage, WM_DESTROY, WNDCLASSEXW, WS_POPUP, WS_VISIBLE};

const fn MAKEINTRESOURCEW(id: u16) -> PWSTR {
    PWSTR(id as *mut u16)
}

fn get_monitor_size(hWnd: HWND) -> RECT {
    let hMonitor = unsafe { MonitorFromWindow(hWnd, MONITOR_DEFAULTTONULL) };
    let mut monInfo = MONITORINFO {
        cbSize: size_of::<MONITORINFO>() as u32,
        ..Default::default()
    };
    unsafe { GetMonitorInfoW(hMonitor, &mut monInfo) };
    monInfo.rcMonitor
}

extern "system" fn wndproc(hWnd: HWND, message: u32, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    match message {
        WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        },
        _ => unsafe { DefWindowProcW(hWnd, message, wParam, lParam) },
    }
}

macro_rules! zerochk {
    ($msg:expr, $e:expr) => {
        if $e == 0 {
            return Err(std::io::Error::last_os_error()).context($msg)
        }
    }
}

fn main() {
    if let Err(e) = main_chk() {
        let mut msg_u16 = format!("{:#}", e).encode_utf16().collect::<Vec<_>>();
        msg_u16.push(0);
        unsafe {
            MessageBoxW(
                None,
                PWSTR(msg_u16.as_mut_ptr()),
                PWSTR(utf16_null!("TouchLock Error").as_mut_ptr()),
                MB_ICONERROR,
            );
        }
        std::process::exit(1);
    }
}

fn main_chk() -> Result<(), anyhow::Error> {
    // We need some parameters that would get passed to wWinMain if we had that as our entry point:
    let hInstance = unsafe { GetModuleHandleW(None) };

    let mut window_name = utf16_null!("TouchLock");
    let window_name_ptr = PWSTR(window_name.as_mut_ptr());

    // ID 1 is the file icon in the EXE.
    let icon = unsafe { LoadIconW(hInstance, MAKEINTRESOURCEW(1)) };

    let wcex = WNDCLASSEXW {
        cbSize: size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wndproc),
        hInstance,
        lpszClassName: window_name_ptr,
        hCursor: unsafe { LoadCursorW(None, &IDC_CROSS) },
        hIcon: icon,
        ..Default::default()
    };

    unsafe { RegisterClassExW(&wcex) };

    let hWnd = unsafe { CreateWindowExW(
        Default::default(),
        window_name_ptr,
        window_name_ptr,
        WS_POPUP | WS_VISIBLE, // WS_OVERLAPPEDWINDOW is a "normal" window; WS_POPUP has no chrome
        CW_USEDEFAULT,
        0,
        CW_USEDEFAULT,
        0,
        None,
        None,
        hInstance,
        null_mut(),
    ) };

    let full = get_monitor_size(hWnd);
    unsafe { SetWindowPos(hWnd, HWND_TOP, full.left, full.top, full.right, full.bottom, Default::default()) };

    unsafe { ShowWindow(hWnd, SW_SHOWDEFAULT) };
    unsafe { UpdateWindow(hWnd) };

    let mut msg: MSG = Default::default();
    while unsafe { GetMessageW(&mut msg, None, 0, 0).as_bool() } {
        unsafe {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
