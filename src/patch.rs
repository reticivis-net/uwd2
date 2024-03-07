use std::fs::File;

pub fn patch(rva: u32) {
    let shell32 = File::open(r"C:\Windows\System32\shell32.dll").unwrap();
}

pub fn kill_explorer() {
    todo!()
}