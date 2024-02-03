use crate::get_guid::get_guid;
pub unsafe fn build_url() -> String {
    let guid = get_guid();
    format!("http://msdl.microsoft.com/download/symbols/shell32.pdb/{guid}/shell32.pdb")
}