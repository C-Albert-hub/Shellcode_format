use std::fs::File;
use std::io::Write;

pub fn write(path: &str, buf: &[u8]) {
    let mut f = File::create(path).unwrap();
    for b in buf {
        writeln!(f, "{:02x}", b).unwrap();
    }
}

pub fn dump(path: &str, buf: &[u8]) {
    let mut f = File::create(path).unwrap();

    for (i, b) in buf.iter().enumerate() {
        if i % 16 == 0 {
            write!(f, "{:08x}: ", i).unwrap();
        }

        write!(f, "{:02x} ", b).unwrap();

        if i % 16 == 15 {
            writeln!(f).unwrap();
        }
    }
}
