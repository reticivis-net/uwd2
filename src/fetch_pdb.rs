use std::io::Read;

pub fn build_url(guid: String) -> String {
    format!("http://msdl.microsoft.com/download/symbols/shell32.pdb/{guid}/shell32.pdb")
}

pub fn fetch(url: String) -> Vec<u8> {
    let resp = ureq::get(url.as_str()).call().unwrap();
    let len: usize = if resp.has("Content-Length") {
        resp.header("Content-Length").unwrap().parse().unwrap()
    } else {
        // last time i checked, the file was about 11.6MB, so this should be fine
        15_000_000
    };

    let mut buf: Vec<u8> = Vec::with_capacity(len);
    resp.into_reader()
        .take(u64::MAX)
        .read_to_end(&mut buf)
        .unwrap();
    buf
}
