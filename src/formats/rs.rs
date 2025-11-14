use std::fs::File;
use std::io::Write;

pub fn write(path: &str, buf: &[u8]) {
    let mut f = File::create(path).unwrap();

    writeln!(f, "pub const SHELLCODE: [u8; {}] = [", buf.len()).unwrap();

    for (i, b) in buf.iter().enumerate() {
        if i % 12 == 0 {
            write!(f, "    ").unwrap();
        }

        write!(f, "0x{:02x}", b).unwrap();

        if i != buf.len() - 1 {
            write!(f, ", ").unwrap();
        }

        if i % 12 == 11 {
            writeln!(f).unwrap();
        }
    }

    writeln!(f, "\n];").unwrap();
}
