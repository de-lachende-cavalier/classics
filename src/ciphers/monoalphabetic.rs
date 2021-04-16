use crate::Cipher;
use std::collections::{HashMap, HashSet};

pub struct MonoalphaCipher {
    key: String,
}

impl MonoalphaCipher {
    // using a Result here is probably a better idea, but to mantain
    // a uniform interface, i'll keep the panic!()
    pub fn new(key: &str) -> Self {
        if MonoalphaCipher::is_alphabet(key) {
            MonoalphaCipher {
                key: key.to_string(),
            }
        } else {
            panic!("Invalid key: the key used in a monoalphabetic cipher must be a permutation of the English alphabet.");
        }
    }

    /// Maps digits to themselves (to simplify their encoding in the
    /// encryption/decryption procedure)
    fn self_map_digits(map: &mut HashMap<char, char>) {
        for d in 0..=9 {
            let ch = std::char::from_digit(d, 10).unwrap();
            map.insert(ch, ch);
        }
    }
    ///
    /// Builds the HashMap that holds the correspondence between the key alphabet
    /// and the original alphabet (only English in our case), used for encryption.
    fn build_map(key: &str) -> HashMap<char, char> {
        let keys = ('A'..='Z').collect::<Vec<char>>();
        let values = key.to_uppercase().chars().collect::<Vec<char>>();
        let mut map: HashMap<char, char> = HashMap::new();

        for (idx, k) in keys.iter().enumerate() {
            map.insert(*k, values[idx]);
        }
        MonoalphaCipher::self_map_digits(&mut map);

        map
    }

    /// Builds the HashMap that holds the inverse correspondence between the key alphabet
    /// and the original alphabet (only English in our case), used for decryption.
    fn build_inverse_map(key: &str) -> HashMap<char, char> {
        let keys = key.to_uppercase().chars().collect::<Vec<char>>();
        let values = ('A'..='Z').collect::<Vec<char>>();
        let mut inverse_map: HashMap<char, char> = HashMap::new();

        for (idx, k) in keys.iter().enumerate() {
            inverse_map.insert(*k, values[idx]);
        }
        MonoalphaCipher::self_map_digits(&mut inverse_map);

        inverse_map
    }
    /// Checks whether a given string constitutes a valid permutation of an alphabet (in our case just
    /// the English alphabet is available)
    fn is_alphabet(s: &str) -> bool {
        let available_letters: HashSet<char> = ('A'..='Z').collect();

        let mut s_letter_count: HashMap<char, u32> = HashMap::new();
        // from https://stackoverflow.com/questions/64178272/what-is-the-idiomatic-rust-way-to-build-a-hashmap-of-character-counts
        let s_vec = s.to_uppercase().chars().collect::<Vec<char>>();
        for ch in s_vec {
            *s_letter_count.entry(ch).or_insert(0) += 1;
        }

        // this allows for duplicates
        let all_keys_are_valid = s_letter_count
            .keys()
            .collect::<Vec<_>>()
            .iter()
            .all(|ch| available_letters.contains(&ch));
        // this returns false if there are duplicates
        let all_letters_appear_once = s_letter_count
            .values()
            .collect::<Vec<_>>()
            .iter()
            .all(|&count| *count == 1);
        let has_26_letters = s.len() == 26;

        if has_26_letters && all_keys_are_valid && all_letters_appear_once {
            true
        } else {
            false
        }
    }
}

impl Cipher for MonoalphaCipher {
    fn encrypt(&self, plaintext: &str) -> String {
        let clean_plaintext = <MonoalphaCipher as Cipher>::clean_input(plaintext);
        let map = MonoalphaCipher::build_map(&self.key);

        clean_plaintext
            .chars()
            .map(|c| *map.get(&c).unwrap())
            .collect::<String>()
    }

    fn decrypt(&self, ciphertext: &str) -> String {
        let clean_ciphertext = <MonoalphaCipher as Cipher>::clean_input(ciphertext);
        let inverse_map = MonoalphaCipher::build_inverse_map(&self.key);

        clean_ciphertext
            .chars()
            .map(|c| *inverse_map.get(&c).unwrap())
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_empty_key() {
        let cipher = MonoalphaCipher::new("");

        cipher.encrypt("Good Kid maad CITY");
    }

    #[test]
    fn test_default_alphabet() {
        let cipher = MonoalphaCipher::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let plaintext = String::from("Hello");

        assert_eq!(plaintext.to_uppercase(), cipher.encrypt(&plaintext));
    }

    #[test]
    fn test_alphabet_detection() {
        // test a correct alphabet
        let possible_alphabet: String = ('a'..='z').collect();
        assert!(MonoalphaCipher::is_alphabet(&possible_alphabet));

        // test a correct but scrambled one
        let possible_alphabet = String::from("klmnopqrstuvwxyzabcdefghij");
        assert!(MonoalphaCipher::is_alphabet(&possible_alphabet));

        // test an obviously incorrect one
        let possible_alphabet = String::from("mbwrtoiu14576184tt9123485--");
        assert!(!MonoalphaCipher::is_alphabet(&possible_alphabet));

        // test a subtly incorrect one (missing an 'f', duplicate 'e')
        let possible_alphabet = String::from("klmnopqrstuvwxyzabcdeeghij");
        assert!(!MonoalphaCipher::is_alphabet(&possible_alphabet));

        // test another subtly incorrect one (the number '1' has been appended to a correct one)
        let possible_alphabet = String::from("klmnopqrstuvwxyzabcdefghij1");
        assert!(!MonoalphaCipher::is_alphabet(&possible_alphabet));
    }

    #[test]
    fn test_numeric_input() {
        let cipher = MonoalphaCipher::new("QHJWOTYRXBKMPIAZEVNULSGDCF");
        let plaintext = "9872465";

        assert_eq!(plaintext.to_string(), cipher.encrypt(plaintext));
    }

    #[test]
    fn test_known_pairs() {
        // from https://cryptii.com
        let cipher = MonoalphaCipher::new("zyxwvutsrqponmlkjihgfedcba");
        let plaintext = "attackatdawn";
        let ciphertext = String::from("zggzxpzgwzdm");

        assert_eq!(ciphertext.to_uppercase(), cipher.encrypt(plaintext));

        assert_eq!(
            <MonoalphaCipher as Cipher>::clean_input(plaintext),
            cipher.decrypt(&ciphertext)
        );

        // same source as above
        let cipher = MonoalphaCipher::new("myxnvestrqpowzlkjihgfudabc");
        let plaintext = "firstman";
        let ciphertext = String::from("erihgwmz");

        assert_eq!(ciphertext.to_uppercase(), cipher.encrypt(plaintext));

        assert_eq!(
            <MonoalphaCipher as Cipher>::clean_input(plaintext),
            cipher.decrypt(&ciphertext)
        );
    }

    #[test]
    #[ignore]
    fn test_correct() {
        let cipher = MonoalphaCipher::new("FJHWOTYRXMKBPIAZEVNULSGDCQ");
        let plaintext = String::from("monoalphabetic");

        for _ in 0..1000 {
            assert_eq!(
                plaintext.to_uppercase(),
                cipher.decrypt(&cipher.encrypt(&plaintext))
            );
        }
    }
}
