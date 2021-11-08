use std::io::Write;

pub fn print_as_binary(v : &Vec<u8>) {
    for i in 0..v.len() {
        if i % 8 == 0 {
            println!();
        }

        print!("{:08b} ", v[i]);
    }

    println!();
    println!();
}

pub fn sha256(input_bytes : &[u8]) -> String {
    const H0 : u32 = 0x6a09e667;
    const H1 : u32 = 0xbb67ae85;
    const H2 : u32 = 0x3c6ef372;
    const H3 : u32 = 0xa54ff53a;
    const H4 : u32 = 0x510e527f;
    const H5 : u32 = 0x9b05688c;
    const H6 : u32 = 0x1f83d9ab;
    const H7 : u32 = 0x5be0cd19;

    const K : [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2  
    ];

    // vector with the bytes

    let mut bytes : Vec<u8> = Vec::new();

    // add the input bytes to the vector

    bytes.extend_from_slice(input_bytes);

    print_as_binary(&bytes);

    // how many bytes to reach a mult of 64 (512 bits)
    
    let to_reach_k64 = 64 - input_bytes.len() % 64;

    println!("len = {}", input_bytes.len());
    println!("to_reach_k64 = {}", to_reach_k64);

    // if more that 8 bytes (64 bits) everythin ok else we add an aditional 64 bytes (512 bits)

    if to_reach_k64 > 8 {
        // add zeros till reach x 64
        
        bytes.resize(input_bytes.len() + to_reach_k64, 0);
    }
    else {
         // add zeros till reach x 64 (+1 block of 64 bytes (512 bits))
        
        bytes.resize(input_bytes.len() + to_reach_k64 + 64, 0);
    }

    print_as_binary(&bytes);

    // change the special bit
    
    bytes[input_bytes.len()] |= 0b10000000;

    print_as_binary(&bytes);

    String::from("awa")
} 

fn main() {
    print!("Enter input: ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("Input failed");

    let bytes = input[0..input.len() - 1].as_bytes();

    let hash = sha256(bytes);

    println!("Hash: {}", hash);
}
