use std::io::Read;

pub fn build_url(guid:String) -> String {
    format!("http://msdl.microsoft.com/download/symbols/shell32.pdb/{guid}/shell32.pdb")
}

pub fn fetch(url:String) -> Vec<u8> {
    let resp = ureq::get(&*url)
        .call().unwrap();
    let len:usize = if resp.has("Content-Length") {
        resp.header("Content-Length")
            .unwrap()
            .parse().unwrap()
    } else {
        usize::MAX
    };

    let mut buf: Vec<u8> = Vec::with_capacity(len);
    resp.into_reader().take(u64::MAX).read_to_end(&mut buf).unwrap();
    buf
}