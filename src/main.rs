mod cipher_decipher;

use std::io;
use std::io::{BufRead, Read};

use cipher_decipher::encipher;
use std::fs::read;

fn main() {
    let mut plaintext = String::new();
    println!("Please enter a string to cipher: ");
    io::stdin()
        .read_line(&mut plaintext)
        .unwrap();
    let mut rotation = String::new();
    println!("Please enter a number to cipher this text by: ");
    io::stdin()
        .read_line(&mut rotation)
        .expect("Input cannot be read");
    let rotation: u8 = rotation.trim().parse().expect("Input cannot be converted to int.");
    println!("{}", encipher(&plaintext, rotation));


}