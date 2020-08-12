pub(crate) fn encipher(plaintext: &str, rotation: u8) -> String {
    let mut ciphertext: Vec<char> = vec![' '; plaintext.len()];
    let to_iter = plaintext.to_ascii_uppercase();
    for(pos, character) in to_iter.chars().enumerate() {
        ciphertext[pos] = if character <= 'Z' && character >= 'A' {
            (((((character as u8) - ('A' as u8)) + rotation) % 26) + ('A' as u8)) as char //if the character is less than A and more than Z, then rotate it
        } else {
            character
        };
    }
    ciphertext.into_iter().collect()
}

/*The only minute difference between this and the cipher is the logic on line 20.*/
pub(crate) fn decipher(plaintext: &str, rotation: u8) -> String {
    let mut ciphertext: Vec<char> = vec![' '; plaintext.len()];
    let to_iter = plaintext.to_ascii_uppercase();
    for(pos, character) in to_iter.chars().enumerate() {
        ciphertext[pos] = if character <= 'Z' && character >= 'A' {
            (((((character as u8) - ('A' as u8)) + (26 - rotation)) % 26) + ('A' as u8)) as char
        } else {
            character
        };
    }
    ciphertext.into_iter().collect()
}