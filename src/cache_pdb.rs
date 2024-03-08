use std::fs;

use crate::constants::*;
use crate::fetch_pdb;
use crate::fetch_pdb::fetch;
use crate::parse_pdb::parse_pdb;

pub fn get_rva(guid: String) -> u32 {
    // quick way to get usable directory
    let dir = data_dir();
    // dbg!(dir);
    // .rva is arbitrary, im storing a single u32 so there isnt exactly a good extension for this
    let pdbpath = dir.join(guid.clone() + ".rva");
    if pdbpath.exists() {
        println!("PDB cached. Reading...");
        let file = fs::read(pdbpath).unwrap();
        u32::from_be_bytes(file.try_into().unwrap())
    } else {
        println!("PDB not found. Fetching...");
        let url = fetch_pdb::build_url(guid);
        let pdbfile = fetch(url);
        println!("Fetched! Parsing...");
        let rva = parse_pdb(pdbfile);
        println!("Parsed! Caching...");
        // remove existing stuff, we dont need to keep around old pdbs that arent valid
        if dir.exists() {
            fs::remove_dir_all(&dir).unwrap();
        }
        // create directories
        fs::create_dir_all(&dir).unwrap();
        // write file
        // yes im implicitly using BE here for consistency rather than NE
        fs::write(pdbpath, rva.to_be_bytes()).unwrap();
        println!("Cached!");
        rva
    }
}
