use windows::Win32::System::Diagnostics::Debug::{IMAGEHLP_SYMBOL64, MAX_SYM_NAME, SYM_LOAD_FLAGS, SYMBOL_INFO, SYMBOL_INFOW, SymEnumerateModules64, SymEnumerateModulesW64, SymEnumerateSymbols64, SymEnumerateSymbolsW64, SymFromName, SymFromNameW, SymGetSymFromName64, SymInitialize, SymLoadModule64, SymLoadModuleExW};
use std::error::Error;
use std::ffi::{c_char, c_void, CStr, CString};
use std::mem::size_of;
use std::ptr;
use windows::core::{s, w};
// use windows::Win32::System::LibraryLoader::GetModuleFileNameA;
// use windows::Win32::System::Threading::GetCurrentThread;
use windows::Win32::Foundation::{HANDLE, MAX_PATH, HINSTANCE, NO_ERROR, WIN32_ERROR, GetLastError, TRUE, BOOL};
use windows::Win32::System::Threading::GetCurrentProcess;


#[link(name = "detours", kind = "static")]
#[link(name = "syelog", kind = "static")]
extern "system" {
    fn DetourTransactionBegin() -> u32;
    fn DetourTransactionAbort() -> u32;
    fn DetourTransactionCommit() -> u32;
    fn DetourUpdateThread(thread: HANDLE) -> u32;
    fn DetourAttach(pointer: *mut *const c_void, detour: *mut *const c_void) -> u32;
    fn DetourDetach(pointer: *mut *const c_void, detour: *mut *const c_void) -> u32;
    fn DetourFindFunction(module: *const c_char, function: *const c_char) -> *const c_void;
}

unsafe extern "system" fn callback(modulename: ::windows::core::PCWSTR, baseofdll: u64, usercontext: *const c_void) -> BOOL {
    println!("{}", modulename.display());
    dbg!(baseofdll, usercontext);
    BOOL(1)
}

unsafe extern "system" fn callback2(symbolname: ::windows::core::PCWSTR, symboladdress: u64, symbolsize: u32, usercontext: *const ::core::ffi::c_void) -> BOOL {

    if symbolname.to_string().unwrap().contains("Wallpaper") {
        println!("{}", symbolname.display());
        dbg!(symboladdress, symbolsize);
    }

    BOOL(1)
}

fn main() {
    unsafe {
        // let result = DetourFindFunction(CString::new(r"\\?\C:\Windows\System32\shell32.dll").unwrap().as_ptr(), CString::new("CDesktopWatermark::s_DesktopBuildPaint").unwrap().as_ptr());
        // dbg!(result);
        let currentprocess = GetCurrentProcess();
        SymInitialize(currentprocess, s!(""), true).expect("initializing failed");
        let name = w!(r"\\?\C:\Windows\System32\shell32.dll");
        let r = SymLoadModuleExW(currentprocess,    // target process
                        HANDLE::default(),        // handle to image - not used
                        name, // name of image file
                         None,        // name of module - not required
                        0,  // base address - not required
                        0,           // size of image - not required
                        None,
            SYM_LOAD_FLAGS::default()
        );
        if r == 0 {
            GetLastError().unwrap();
        }
        SymEnumerateSymbolsW64(currentprocess, 6442450944, Some(callback2), None).unwrap();
        // SymEnumerateModulesW64(currentprocess, Some(callback), None).unwrap();
        // let mut symbol = IMAGEHLP_SYMBOL64 {
        //     SizeOfStruct: size_of::<IMAGEHLP_SYMBOL64>() as u32,
        //     MaxNameLength: MAX_SYM_NAME,
        //     ..Default::default()
        // };
        // SymGetSymFromName64(currentprocess, s!("CDesktopWatermark::s_DesktopBuildPaint"), &mut symbol as *mut IMAGEHLP_SYMBOL64).unwrap();
        // dbg!(symbol);
    }
}