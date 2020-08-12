mod cipher_decipher;

use std::io;
use std::io::{BufRead, Read};

use cipher_decipher::encipher;
use std::fs::read;
use crate::cipher_decipher::decipher;

fn main() {
    let decipher = String::from("d\n");
    let encipher = String::from("e\n");
    let mut choice = String::new();
    println!("Do you want to decipher(d) or encipher(e)?");
    io::stdin()
        .read_line(&mut choice)
        .unwrap();
    if choice.eq(&decipher) {
        user_choice(decipher);
    } else if choice.eq(&encipher) {
        user_choice((encipher));
    } else {
        println!("That's all folks!");
    }
}

fn user_choice(choice: String) {
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
    if choice.eq("d\n") {
        println!("{}", decipher(&plaintext, rotation))
    } else if choice.eq("e\n") {
        println!("{}", encipher(&plaintext, rotation));
    } else {
        eprintln!("Error");
    }

}