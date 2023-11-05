use std::io::{Cursor, Read};
use tar::Archive;

const ARCHIVE_CONTENT: &[u8] = include_bytes!("../work/archive.tar");

fn main() {
    let mut archive = Archive::new(Cursor::new(ARCHIVE_CONTENT));
    let entries = archive.entries().expect("read entries").map(Result::unwrap);
    for mut entry in entries {
        let mut content = String::new();
        entry.read_to_string(&mut content).expect("read content");
        let path = entry.path().expect("get path");
        eprintln!("{path:?} -> {content}");
    }
}
