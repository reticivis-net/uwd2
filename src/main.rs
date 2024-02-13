use std::io;
use pdb::FallibleIterator;
use crate::cache_pdb::get_rva;
use crate::explorer_modinfo::get_guid;

mod fetch_pdb;
mod explorer_modinfo;
mod parse_pdb;
mod cache_pdb;
mod inject;

fn main() {
    let guid;
    unsafe {
        guid = get_guid();
    }
    let rva = get_rva(guid);
    unsafe { inject::inject(rva); }
    println!("{rva:#x}");
    // dbg!(pdbfile);
}
