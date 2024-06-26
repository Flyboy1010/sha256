mod sha;

use std::io::Write;

fn main() {
    print!("Enter input: ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("Input failed");

    let bytes = input[0..input.len() - 2].as_bytes();

    let hash = sha::sha256(bytes);
    println!("Hash: {}", hash.to_string());
}
