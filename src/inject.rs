use crate::constants::*;
use crate::explorer_modinfo::{get_explorer_handle, get_shell32_offset};
use std::ffi::c_void;
use windows::core::imp::CloseHandle;
use windows::core::s;
use windows::Win32::Foundation::{GetLastError, LPARAM, WPARAM};
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::UI::Shell::{
    SHChangeNotify, SHGetSetSettings, SHCNE_ASSOCCHANGED, SHCNF_IDLIST, SHELLSTATEA, SSF_HIDEICONS,
    SSF_MASK,
};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, GetWindow, SendMessageA, SendMessageTimeoutA, GW_CHILD, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_COMMAND, WM_SETTINGCHANGE, GetWindowInfo, WINDOWINFO, WS_VISIBLE};

pub unsafe fn inject(rva: u32) {
    println!("Getting shell32 offset...");
    let offset = get_shell32_offset();
    println!("Offset of shell32 inside explorer.exe is {offset:#x}");
    let explorerhandle = get_explorer_handle();
    println!("Injecting ret...");
    let buffer: [u8; 1] = [RET];
    // write return instruction to address of function, effectively disabling it
    WriteProcessMemory(
        explorerhandle,
        // offset is position of dll inside explorer.exe, rva is position of func inside dll
        (offset + rva as u64) as *const c_void,
        &buffer as *const u8 as *const c_void,
        1,
        None,
    )
    .unwrap();
    println!("Injected!");
    CloseHandle(explorerhandle.0);
}

pub unsafe fn refresh() {
    println!("Refreshing desktop...");
    let hWnd = GetWindow(FindWindowA(s!("Progman"), s!("Program Manager")), GW_CHILD);

    // check if desktop icons are visible
    let hWnd2 = GetWindow(hWnd, GW_CHILD);
    let mut wi = WINDOWINFO::default();
    wi.cbSize = std::mem::size_of::<WINDOWINFO>() as u32;
    GetWindowInfo(hWnd2, &mut wi as *mut _).unwrap();
    let visible = wi.dwStyle & WS_VISIBLE == WS_VISIBLE;

    if visible {
        // i have no idea why this works
        // "A file type association has changed" causes the desktop to refresh
        // which makes the watermark go away so whatever it works
        SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, None, None);
    } else {
        // if icons are hidden, no refreshing or anything will work, so just unhide and rehide the icons
        SendMessageA(hWnd, WM_COMMAND, WPARAM(0x7402), LPARAM::default());
        SendMessageA(hWnd, WM_COMMAND, WPARAM(0x7402), LPARAM::default());
    }

    println!("{:b}", wi.dwStyle.0);
    // SendMessageA(hWnd, WM_COMMAND, WPARAM(0x7402), LPARAM::default());
    // dbg!(ss.fHideIcons);
    println!("Refreshed!")
}
