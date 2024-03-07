use crate::constants;
use crate::constants::*;
use directories::ProjectDirs;
use std::ffi::OsStr;
use std::fs::{read, write};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use winreg::enums::HKEY_CLASSES_ROOT;
use winreg::RegKey;

const DEFAULT_SHELL32_REG_PATH: &str = r"%SystemRoot%\system32\shell32.dll";
pub fn patch(rva: u32) {
    let dir = data_dir();
    let path = dir.join("shell32.patched.dll");
    // read file
    let mut s32 = read(SHELL32_PATH).expect("Unable to read shell32.dll");
    // patch
    s32[rva as usize] = RET;
    // write
    write(&path, s32).expect("Unable to write patched file");
    // evil registry hack to point windows to our file
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    let key = hkcr
        .open_subkey(r"CLSID\{0010890e-8789-413c-adbc-48f5b511b3af}\InProcServer32")
        .unwrap();
    key.set_value("", &path.as_os_str()).unwrap()
}

pub fn unpatch() {
    // undo evil registry hack
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    let key = hkcr
        .open_subkey(r"CLSID\{0010890e-8789-413c-adbc-48f5b511b3af}\InProcServer32")
        .unwrap();
    key.set_value("", &OsStr::new(DEFAULT_SHELL32_REG_PATH))
        .unwrap()
}

pub fn kill_explorer() {
    // i know it's a duplicate code but i can't for the life of me get it in a function
    // it returns an &Process which can't be cloned or copied or moved
    sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::new().with_processes(sysinfo::ProcessRefreshKind::everything()),
    )
        // get explorer
        .processes()
        .values()
        .find(|proc| {
            if let Some(p) = proc.exe() {
                p == Path::new(r"C:\Windows\explorer.exe")
            } else {
                false
            }
        })
        .unwrap()
        .kill();
}
