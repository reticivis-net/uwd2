use windows::Win32::System::Diagnostics::Debug::{IMAGEHLP_MODULE64, IMAGEHLP_SYMBOL64, MAX_SYM_NAME, SYM_LOAD_FLAGS, SYMBOL_INFO, SYMBOL_INFOW, SymEnumerateModules64, SymEnumerateModulesW64, SymEnumerateSymbols64, SymEnumerateSymbolsW64, SymFromName, SymFromNameW, SymGetModuleInfo64, SymGetSymFromName64, SymInitialize, SymLoadModule64, SymLoadModuleExW, SYMOPT_DEFERRED_LOADS, SYMOPT_UNDNAME, SymSetOptions};
use std::error::Error;
use std::ffi::{c_char, c_void, CStr, CString};
use std::mem::size_of;
use std::ptr;
use windows::core::{PCSTR, s, w};
// use windows::Win32::System::LibraryLoader::GetModuleFileNameA;
// use windows::Win32::System::Threading::GetCurrentThread;
use windows::Win32::Foundation::{HANDLE, MAX_PATH, HINSTANCE, NO_ERROR, WIN32_ERROR, GetLastError, TRUE, BOOL};
use windows::Win32::System::Threading::GetCurrentProcess;
use windows::Win32::System::LibraryLoader::{LOAD_LIBRARY_FLAGS, LoadLibraryExA};


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
        // GetLastError().unwrap();
        let currentprocess = GetCurrentProcess();
        SymSetOptions(SYMOPT_UNDNAME | SYMOPT_DEFERRED_LOADS);
        SymInitialize(currentprocess, s!("SRV*"), true).expect("initializing failed");
        let name = s!(r"C:\Windows\System32\shell32.dll");
        let module = LoadLibraryExA(name, HANDLE::default(), LOAD_LIBRARY_FLAGS::default()).unwrap();
        dbg!(module);

        let r = SymLoadModule64(currentprocess,    // target process
                        HANDLE::default(),        // handle to image - not used
                        name, // name of image file
                         PCSTR::null(),        // name of module - not required
                        module.0 as u64,  // base address - not required
                        0,           // size of image - not required
        );
        if r == 0 {
            GetLastError().unwrap();
        }
        let mut modinfo = IMAGEHLP_MODULE64 {
            SizeOfStruct: size_of::<IMAGEHLP_MODULE64>() as u32,
            ..Default::default()
        };
        // dbg!(modinfo);
        SymGetModuleInfo64(currentprocess, module.0 as u64, &mut modinfo as *mut IMAGEHLP_MODULE64).unwrap();
        dbg!(modinfo);
        let modname = PCSTR::from_raw(&modinfo.ModuleName as *const u8);
        println!("{}", modname.display()); // is shell32
        // SymEnumerateSymbolsW64(currentprocess, 6442450944, Some(callback2), None).unwrap();
        // SymEnumerateModulesW64(currentprocess, Some(callback), None).unwrap();
        let mut symbol = SYMBOL_INFO {
            SizeOfStruct: size_of::<SYMBOL_INFO>() as u32,
            MaxNameLen: MAX_SYM_NAME,
            ..Default::default()
        };
        SymFromName(currentprocess, s!("shell32!CDesktopWatermark::s_DesktopBuildPaint"), &mut symbol as *mut SYMBOL_INFO).unwrap();
        dbg!(symbol);
    }
}