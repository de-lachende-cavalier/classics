use super::utils::clean_input;
// Vigenere is just repeated Caesar after all
use super::shift::shift_by;

pub fn encrypt(plaintext: &str, key: &str) -> String {
    let clean_plaintext = clean_input(plaintext);
    let upper_key = key.to_uppercase();
    let mut ciphertext: Vec<char> = Vec::new();

    for (idx, ch) in clean_plaintext.chars().enumerate() {
        let ch_k = upper_key.as_bytes()[(idx % key.len())];
        let shift = (ch_k as u32 - 'A' as u32) as i8;

        ciphertext.push(shift_by(shift, ch));
    }

    ciphertext.iter().collect::<String>()
}

pub fn decrypt(ciphertext: &str, key: &str) -> String {
    let clean_ciphertext = clean_input(ciphertext);
    let upper_key = key.to_uppercase();
    let mut plaintext: Vec<char> = Vec::new();

    for (idx, ch) in clean_ciphertext.chars().enumerate() {
        let ch_k = upper_key.as_bytes()[(idx % key.len())];
        let shift = (ch_k as u32 - 'A' as u32) as i8;

        plaintext.push(shift_by(-shift, ch));
    }

    plaintext.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_empty_key() {
        let plaintext = String::from("Hello");

        encrypt(&plaintext, "");
    }

    #[test]
    fn test_key_longer_than_pt() {
        let plaintext = String::from("shorttext");
        let key = "testinglongkey";
        let ciphertext = String::from("llgkbgkih");

        assert_eq!(ciphertext.to_uppercase(), encrypt(&plaintext, key));
    }

    #[test]
    fn test_known_pairs() {
        // from https://en.wikipedia.org/wiki/Vigenere_cipher
        let plaintext = String::from("attackatdawn");
        let key = "lemon";
        let ciphertext = String::from("LXFOPVEFRNHR");

        assert_eq!(ciphertext, encrypt(&plaintext, key));

        // from https://cryptii.com/
        let plaintext = String::from("firstman");
        let key = "cryptii";
        let ciphertext = String::from("hzphmuip");

        assert_eq!(ciphertext.to_uppercase(), encrypt(&plaintext, key));
    }

    #[test]
    fn test_correct() {
        let plaintext = String::from("attackatdawn");
        let key = "lemon";

        for _ in 0..10 {
            assert_eq!(
                plaintext.to_uppercase(),
                decrypt(&encrypt(&plaintext, key), key)
            );
        }
    }
}
