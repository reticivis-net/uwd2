mod get_guid;
mod fetch_pdb;

fn main() {
    unsafe { println!("{}", fetch_pdb::build_url()); }
}
