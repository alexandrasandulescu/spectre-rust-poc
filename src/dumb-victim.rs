#![feature(asm)]
use std::io::{self, BufRead};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod utils;
use utils::busy_waiting;

mod spec;

fn specv1_send(index: char) -> u64 {
    // 256 bytes as offset; 256 entries => 16 pages
    // use /lib/x86_64-linux-gnu/ld-2.23.so as shared zone since it should
    // not be used after process startup
    let result: u64;

    unsafe {
        asm!{"
            movzx rax, al
            shl rax, 9
            add rax, {specv1_base}
            mov {result}, [rax]
            lfence
            ",
            specv1_base = in(reg) spec::SPECV1_BASE,
            result = out(reg) result,
            in("al") index as u8,
        };
    }

    result
}

// Rust noob. Code from here:
// https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
fn read_flag(path: &Path) -> String {
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => s,
    }
}

fn main() {

    let path = Path::new("flag.txt");

    let secret = read_flag(path);
    let secret_chars: Vec<char> = secret.chars().collect();

    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let line = it.next().unwrap();
    let input_chars: Vec<char> = line.unwrap().chars().collect();

    if input_chars.len() != secret_chars.len() {
        "Wrong input".to_string();
        return;
    }

    busy_waiting(1000);

    for cindex in 0..secret_chars.len() {
        if secret_chars[cindex] != input_chars[cindex] {
            specv1_send(secret_chars[cindex]);
            // delay exit
            busy_waiting(20000);
            panic!("{}", "Wrong input");
        }
    }

    "Congrats. You got the flag".to_string();
}
