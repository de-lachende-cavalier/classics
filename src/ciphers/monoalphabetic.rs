use super::utils::clean_input;
use std::collections::HashMap;

/// Maps digits to themselves (to simplify their encoding in the
/// encryption/decryption procedure)
fn self_map_digits(map: &mut HashMap<char, char>) {
    for d in 0..=9 {
        let ch = std::char::from_digit(d, 10).unwrap();
        map.insert(ch, ch);
    }
}

/// Builds the HashMap that holds the correspondence between the key alphabet
/// and the original alphabet (only English in our case), used for encryption.
fn build_map(key: &str) -> HashMap<char, char> {
    let keys = ('A'..='Z').collect::<Vec<char>>();
    let values = key.to_uppercase().chars().collect::<Vec<char>>();
    let mut map: HashMap<char, char> = HashMap::new();

    for (idx, k) in keys.iter().enumerate() {
        map.insert(*k, values[idx]);
    }
    self_map_digits(&mut map);

    map
}

/// Builds the HashMap that holds the inverse correspondence between the key alphabet
/// and the original alphabet (only English in our case), used for decryption.
fn build_inverse_map(key: &str) -> HashMap<char, char> {
    let keys = key.chars().collect::<Vec<char>>();
    let values = ('A'..='Z').collect::<Vec<char>>();
    let mut inverse_map: HashMap<char, char> = HashMap::new();

    for (idx, k) in keys.iter().enumerate() {
        inverse_map.insert(*k, values[idx]);
    }
    self_map_digits(&mut inverse_map);

    inverse_map
}

pub fn encrypt(plaintext: &str, key: &str) -> String {
    let clean_plaintext = clean_input(plaintext);
    let map = build_map(key);

    clean_plaintext
        .chars()
        .map(|c| *map.get(&c).unwrap())
        .collect::<String>()
}

pub fn decrypt(ciphertext: &str, key: &str) -> String {
    let clean_ciphertext = clean_input(ciphertext);
    let inverse_map = build_inverse_map(key);

    clean_ciphertext
        .chars()
        .map(|c| *inverse_map.get(&c).unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_empty_key() {
        let plaintext = String::from("Good Kid maad CITY");

        encrypt(&plaintext, "");
    }

    #[test]
    fn test_default_alphabet() {
        let plaintext = String::from("Hello");
        let key = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

        assert_eq!(plaintext.to_uppercase(), encrypt(&plaintext, key));
    }

    #[test]
    fn test_numeric_input() {
        let plaintext = String::from("123445678890");
        let key = "QHJWOTYRXBKMPIAZEVNULSGDCF";

        assert_eq!(plaintext, encrypt(&plaintext, key));
    }

    #[test]
    fn test_known_pairs() {
        // from https://cryptii.com
        let plaintext = String::from("attackatdawn");
        let key = "zyxwvutsrqponmlkjihgfedcba";
        let ciphertext = String::from("zggzxpzgwzdm");

        assert_eq!(ciphertext.to_uppercase(), encrypt(&plaintext, key));

        let plaintext = String::from("firstman");
        let key = "myxnvestrqpowzlkjihgfudabc";
        let ciphertext = String::from("erihgwmz");

        assert_eq!(ciphertext.to_uppercase(), encrypt(&plaintext, key));
    }

    #[test]
    fn test_correct() {
        let plaintext = String::from("monoalphabetic");
        let key = "FJHWOTYRXMKBPIAZEVNULSGDCQ";

        // 10 is arbitrary
        for _ in 0..10 {
            assert_eq!(
                plaintext.to_uppercase(),
                decrypt(&encrypt(&plaintext, key), key)
            );
        }
    }
}
