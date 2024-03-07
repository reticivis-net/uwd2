use std::env;

use pdb::FallibleIterator;

use crate::cache_pdb::get_rva;
use crate::explorer_modinfo::get_guid;

mod cache_pdb;
mod constants;
mod elevate;
mod explorer_modinfo;
mod fetch_pdb;
mod inject;
mod parse_pdb;
mod patch;

fn prog() -> String {
    // modified from https://stackoverflow.com/a/58113997/9044183
    env::current_exe()
        .unwrap()
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap()
}

fn help() {
    println!(
        include_str!("../help.txt"),
        env!("CARGO_PKG_VERSION"),
        prog()
    )
}

fn rva() -> u32 {
    let guid;
    unsafe {
        guid = get_guid();
    }
    let rva = get_rva(guid);
    println!("RVA is {rva:#x}");
    rva
}

fn patch() {
    unsafe {
        elevate::elevate_if_needed();
        patch::patch(rva());
        patch::kill_explorer();
    }
}

fn unpatch() {
    unsafe {
        elevate::elevate_if_needed();
        patch::unpatch();
        patch::kill_explorer();
    }
}

fn inject() {
    unsafe {
        inject::inject(rva());
        inject::refresh();
    }
}
fn main() {
    match env::args().collect::<Vec<String>>().get(1) {
        None => inject(),
        Some(arg) => match arg.as_str() {
            "patch" => patch(),
            "inject" => inject(),
            "unpatch" => patch(),
            "help" => help(),
            "about" => {
                println!(include_str!("../about.txt"), env!("CARGO_PKG_VERSION"))
            }
            err => {
                eprintln!(
                    "Invalid argument `{err}`. Run `{} help` to see all commands.",
                    prog()
                )
            }
        },
    }

    // dbg!(pdbfile);
}
