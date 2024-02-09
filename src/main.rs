use crate::fetch_pdb::fetch;
use crate::get_guid::get_guid;

mod fetch_pdb;
mod get_guid;

fn main() {
    let guid;
    unsafe {
        guid = get_guid();
    }
    let url = fetch_pdb::build_url(guid);
    let pdbfile = fetch(url);
    // dbg!(pdbfile);
}
