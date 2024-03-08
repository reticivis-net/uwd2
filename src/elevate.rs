use std::env;
use std::ffi::{c_void, OsStr, OsString};
use windows::core::HSTRING;
use windows::Win32::Foundation::{GetLastError, HANDLE, INVALID_HANDLE_VALUE};
use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use windows::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_NORMAL;

unsafe fn is_admin() -> bool {
    // get process
    let proc = GetCurrentProcess();
    // default value just to have something
    let mut handle: HANDLE = INVALID_HANDLE_VALUE;
    // open handle thing
    OpenProcessToken(proc, TOKEN_QUERY, &mut handle as *mut HANDLE).unwrap();
    // set up vars
    let mut elevation = TOKEN_ELEVATION::default();
    let size: u32 = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
    let mut ret_size: u32 = size;
    // get info
    GetTokenInformation(
        handle,
        TokenElevation,
        Some(&mut elevation as *mut _ as *mut c_void),
        size,
        &mut ret_size,
    )
        .unwrap();
    // seems to be 0 = not admin, 1 = admin
    elevation.TokenIsElevated != 0
}

unsafe fn get_self_path() -> HSTRING {
    // prepare buffer
    const BUF_SIZE: usize = 0x10_000; // overkill but shut up
    let mut self_path_buf: [u16; BUF_SIZE] = [0; BUF_SIZE];
    // get path
    let length = GetModuleFileNameW(None, &mut self_path_buf);
    // error handling isnt automatic for this function for some reason
    GetLastError().ok().unwrap();
    // create hstring from buffer
    let self_path = HSTRING::from_wide(&self_path_buf[..length as usize]).unwrap();
    self_path
}

unsafe fn elevate() {
    // get path to self
    let self_path = get_self_path();
    // run it as admin
    ShellExecuteW(
        None,
        // https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecutew#runas
        &HSTRING::from("runas"), // run as admin
        &self_path,
        // pass args
        &HSTRING::from(
            env::args_os()
                .skip(1)
                .collect::<Vec<OsString>>()
                .join(OsStr::new(" ")),
        ),
        None,
        SW_NORMAL,
    );
    // error handling isnt automatic for this function for some reason
    GetLastError().ok().unwrap();
}

pub fn elevate_if_needed() {
    // if program is not running as admin, spawn new process as admin and exit
    // "reincarnate" as admin-privileged process
    let admin = unsafe { is_admin() };
    if !admin {
        println!("Not running as admin! Requesting elevation...");
        unsafe {
            elevate();
            println!("Elevated new process. Goodbye!");
        }
        std::process::exit(0);
    }
}
