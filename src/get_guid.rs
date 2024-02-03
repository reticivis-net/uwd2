use std::mem::size_of;
use windows::core::{s, PCSTR, GUID};
use windows::Win32::Foundation::{GetLastError, HANDLE};
use windows::Win32::System::Diagnostics::Debug::{
    SymGetModuleInfo64, SymInitialize, SymLoadModuleEx, SymSetOptions, IMAGEHLP_MODULE64,
    SYMOPT_UNDNAME, SYM_LOAD_FLAGS,
};
use windows::Win32::System::LibraryLoader::{LoadLibraryExA, LOAD_LIBRARY_FLAGS};
use windows::Win32::System::Threading::GetCurrentProcess;

pub unsafe fn get_guid() -> String {
    let currentprocess = GetCurrentProcess();
    SymInitialize(currentprocess, PCSTR::null(), true).expect("initializing failed");
    SymSetOptions(SYMOPT_UNDNAME);
    let name = s!(r"C:\Windows\System32\shell32.dll");
    let module = LoadLibraryExA(name, HANDLE::default(), LOAD_LIBRARY_FLAGS::default()).unwrap();
    let r = SymLoadModuleEx(
        currentprocess,    // target process
        HANDLE::default(), // handle to image - not used
        name,              // name of image file
        PCSTR::null(),     // name of module - not required
        module.0 as u64,   // base address - not required
        0,                 // size of image - not required
        None,
        SYM_LOAD_FLAGS::default(),
    );
    if r == 0 {
        GetLastError().unwrap();
    }
    let mut modinfo = IMAGEHLP_MODULE64 {
        SizeOfStruct: size_of::<IMAGEHLP_MODULE64>() as u32,
        ..Default::default()
    };
    SymGetModuleInfo64(
        currentprocess,
        module.0 as u64,
        &mut modinfo as *mut IMAGEHLP_MODULE64,
    )
    .unwrap();
    let sig = modinfo.PdbSig70.to_u128();
    let age = modinfo.PdbAge;
    format!("{sig:032X}{age:X}")
}
