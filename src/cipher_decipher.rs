pub(crate) fn encipher(plaintext: &str, rotation: u8) -> String {
    let mut ciphertext: Vec<char> = vec![' '; plaintext.len()];
    let to_iter = plaintext.to_ascii_uppercase();
    for(idx, character) in to_iter.chars().enumerate() {
        ciphertext[idx] = if character <= 'Z' && character >= 'A' {
            (((((character as u8) - ('A' as u8)) + rotation) % 26) + ('A' as u8)) as char //if the character is less than A and more than Z, then rotate it
        } else {
            character
        };
    }
    ciphertext.into_iter().collect()
}