use crate::constants::*;
use crate::explorer_modinfo::{get_explorer_handle, get_shell32_offset};
use std::ffi::c_void;
use windows::core::imp::CloseHandle;
use windows::Win32::Foundation::{GetLastError, LPARAM, WPARAM};
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::UI::Shell::{SHChangeNotify, SHCNE_ASSOCCHANGED, SHCNF_IDLIST, SHELLSTATEA, SHGetSetSettings, SSF_HIDEICONS, SSF_MASK};
use windows::Win32::UI::WindowsAndMessaging::{HWND_BROADCAST, SendMessageTimeoutA, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE};

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
    // i have no idea why this works
    // "A file type association has changed" causes the desktop to refresh
    // which makes the watermark go away so whatever it works
    // SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, None, None);

    let mut ss = SHELLSTATEA::default();
    SHGetSetSettings(Some(&mut ss as *mut _), SSF_MASK(u32::MAX), false);
    GetLastError().ok().unwrap();
    // dbg!(ss.fHideIcons);
    let copy = std::mem::transmute::<i32, u32>(ss._bitfield1);
    let icons_hidden = copy & (1 << 12) != 0;
    println!("10010100000110111\n{}", icons_hidden);
    println!("{:b}", copy);
    // if icons_hidden {
    //     // there is no way to refresh explorer if the icons are hidden i tried so just show and hide
    //     ss._bitfield1 &= !(1 << 12);
    //     SHGetSetSettings(Some(&mut ss as *mut _), SSF_MASK(u32::MAX), true);
    //     GetLastError().ok().unwrap();
    // 
    //     // ss._bitfield1 |= (1 << 12);
    //     // SHGetSetSettings(Some(&mut ss as *mut _), SSF_MASK(u32::MAX), true);
    //     // GetLastError().ok().unwrap();
    // } else{
    //     // equivalent to right click and refresh
    //     SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, None, None);
    // }
}
