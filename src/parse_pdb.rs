use pdb::{FallibleIterator, Rva};
use std::io::Cursor;

pub fn parse_pdb(pdbfile: Vec<u8>) -> u32 {
    let pdbreader = Cursor::new(pdbfile);
    let mut shell32 = pdb::PDB::open(pdbreader).unwrap();
    let symbol_table = shell32.global_symbols().unwrap();
    let address_map = shell32.address_map().unwrap();
    for symbol in symbol_table.iter().iterator().flatten() {
        let data = symbol.parse().unwrap();
        if let pdb::SymbolData::Public(d) = data {
            // i'd be surprised if there's any false positives here and i dont want to get cocky with name unmangling
            if d.name.to_string().contains("s_DesktopBuildPaint") && d.function {
                // dbg!(d);
                let rva = d.offset.to_rva(&address_map).unwrap();
                // dbg!(rva);
                return rva.0;
            }
        }
    }
    panic!("ermmm what the derp... (cant find symbol in pdb)");
}
