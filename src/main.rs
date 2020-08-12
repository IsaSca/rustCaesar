mod cipher_decipher;

use std::io;
use std::io::BufRead;

use cipher_decipher::encipher;

fn main() {
    println!("Hello, world!");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", encipher(&line.unwrap(), 3));
    }

}