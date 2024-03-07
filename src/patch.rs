use std::fs;
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;
use directories::ProjectDirs;
use crate::constants;
use crate::constants::*;

pub fn patch(rva: u32) {
    let dir = data_dir();
    let path = dir.join("shell32.unpatched.dll");
    fs::copy(SHELL32_PATH, path).expect("Unable to save copy of shell32.");
    let mut shell32 = File::open(SHELL32_PATH).expect("Unable to open file");
    shell32
        .seek(SeekFrom::Start(rva as u64))
        .expect("Unable to seek.");
    let buffer: [u8; 1] = [RET];
    shell32.write(&buffer).expect("Unable to patch file.");
}

pub fn unpatch() {
    let dir = data_dir();
    let path = dir.join("shell32.unpatched.dll");
    fs::copy(path, SHELL32_PATH).expect("Unable to save copy of shell32.");
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
