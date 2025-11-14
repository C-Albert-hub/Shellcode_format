use std::fs::File;
use std::io::Write;

pub fn write(path: &str, buf: &[u8]) {
    let mut f = File::create(path)
        .unwrap_or_else(|_| panic!("[-] Cannot write {}", path));

    f.write_all(buf).unwrap();
}
