use std::ffi::c_void;
use windows::core::imp::CloseHandle;
use crate::explorer_modinfo::{get_explorer_handle, get_shell32_offset};
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;

pub unsafe fn inject(rva: u32) {
    println!("Getting shell32 offset...");
    let offset = get_shell32_offset();
    println!("Offset of shell32 inside explorer.exe is {offset:#x}");
    let explorerhandle = get_explorer_handle();
    println!("Injecting ret...");
    // ret instruction
    let buffer: [u8; 1] = [0xC3];
    // write return instruction to address of function, effectively disabling it
    WriteProcessMemory(
        explorerhandle,
        // offset is position of dll inside explorer.exe, rva is position of func inside dll
        (offset + rva as u64) as *const c_void,
        &buffer as *const u8 as *const c_void,
        1,
        None
    )
    .unwrap();
    println!("Injected!");
    CloseHandle(explorerhandle.0);
}
