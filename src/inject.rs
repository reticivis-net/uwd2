use std::ffi::c_void;
use crate::explorer_modinfo::{get_explorer_handle, get_shell32_offset};
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;

pub unsafe fn inject(rva: u32) {
    let offset = get_shell32_offset();
    let explorerhandle = get_explorer_handle();
    // ret instruction
    let buffer: [u8; 1] = [0xC3];
    WriteProcessMemory(
        explorerhandle,
        (offset + rva as u64) as *const c_void,
        &buffer as *const u8 as *const c_void,
        1,
        None
    )
    .unwrap();
}
