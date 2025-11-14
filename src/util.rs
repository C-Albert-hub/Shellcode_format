use std::{fs::File, io::Read};

// ANSI colors
const RED:    &str = "\x1b[31m";
const GREEN:  &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE:   &str = "\x1b[34m";
const BOLD:   &str = "\x1b[1m";
const RESET:  &str = "\x1b[0m";

// helper to wrap message
fn color(c: &str, msg: &str) -> String {
    format!("{c}{msg}{RESET}")
}

// output helpers
pub fn info(msg: &str) {
    println!("{} {}", color(BLUE,  "[*]"), msg);
}

pub fn good(msg: &str) {
    println!("{} {}", color(GREEN, "[+]"), msg);
}

pub fn warn(msg: &str) {
    println!("{} {}", color(YELLOW, "[!]"), msg);
}

pub fn bad(msg: &str) {
    println!("{} {}", color(RED,  "[-]"), msg);
}

// optional: title banner
pub fn banner() {
    println!(
        "{}{}\n{}\n{}",
        BOLD,
        "bin2payload — Binary → Payload Generator",
        "M4c4r0n1",
        RESET
    );
}

pub fn read_binary(path: &str) -> Vec<u8> {
    let mut f = File::open(path)
        .unwrap_or_else(|_| panic!("[-] Failed to open {}", path));

    let mut buf = Vec::new();
    f.read_to_end(&mut buf)
        .expect("[-] Failed to read file");

    buf
}