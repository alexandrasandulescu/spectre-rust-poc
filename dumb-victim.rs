use std::io::{self, BufRead};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn specv1_gadget(index: usize) -> i64 {
    // 256 bytes as offset; 256 entries => 16 pages
    let array: [i64; 16 << 12] = [0; 16 << 12];

    return array[index];
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
        Ok(_) => return s,
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
        println!("{}", "Wrong input");
        return;
    }

    specv1_gadget(10);
    for cindex in 0..=secret_chars.len() - 1 {
        if secret_chars[cindex] != input_chars[cindex] {
            println!("{}", "Wrong input");
            return;
        }
    }

    println!("{}", "Congrats. You got the flag");
}
