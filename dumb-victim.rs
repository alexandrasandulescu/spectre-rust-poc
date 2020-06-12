use std::io::{self, BufRead};

fn specv1_gadget(index: usize) -> i64 {
    // 256 bytes as offset; 256 entries => 16 pages
    let array: [i64; 16 << 12] = [0; 16 << 12];

    return array[index];
}

fn main() {
    specv1_gadget(10);

    let secret: &'static str = "FLAG{e848c897e0378982a1e630832a3588168e3d0cc92ba69cf4425f4363de5789c6}";
    let secret_chars: Vec<char> = secret.chars().collect();

    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let line = it.next().unwrap();
    let input_chars: Vec<char> = line.unwrap().chars().collect();

    if input_chars.len() != secret_chars.len() {
        println!("{}", "Wrong input");
        return;
    }

    for cindex in 0..=secret_chars.len() - 1 {
        if secret_chars[cindex] != input_chars[cindex] {
            println!("{}", "Wrong input");
            return;
        }
    }

    println!("{}", "Congrats. You got the flag");
}
