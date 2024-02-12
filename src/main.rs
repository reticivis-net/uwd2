use std::io;
use std::io::Cursor;
use pdb::FallibleIterator;
use pdb::PublicSymbol;
use crate::cache_pdb::get_rva;
use crate::fetch_pdb::{fetch};
use crate::get_guid::get_guid;
use crate::parse_pdb::parse_pdb;

mod fetch_pdb;
mod get_guid;
mod parse_pdb;
mod cache_pdb;

fn main() {
    let guid;
    unsafe {
        guid = get_guid();
    }
    let rva = get_rva(guid);
    println!("{rva:#x}");
    // dbg!(pdbfile);
}
