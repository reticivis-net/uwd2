use std::path::PathBuf;

use directories::ProjectDirs;

pub fn data_dir() -> PathBuf {
    let binding = ProjectDirs::from("net", "reticivis", "UWD2").unwrap();
    let dir = binding.data_dir();
    dir.to_owned()
}

pub const SHELL32_PATH: &str = r"C:\Windows\System32\shell32.dll";

// ret instruction
// NOTE: THIS IS FOR X86, WILL NOT WORK ON ARM
pub const RET: u8 = 0xC3;
