use std::fs::File;
use std::io::Write;

pub fn write(path: &str, buf: &[u8]) {
    let encoded = base64::encode(buf);

    let mut f = File::create(path)
        .unwrap_or_else(|_| panic!("[-] Cannot write {}", path));
    
    writeln!(f, "{}", encoded).unwrap();
}
