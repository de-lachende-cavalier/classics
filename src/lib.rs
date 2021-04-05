mod ciphers;

use ciphers::monoalphabetic;
use ciphers::shift;
use ciphers::vigenere;

use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

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

/// Gets data based on the CLI args provided (if a file has been specified that one is used
/// otherwise use stdin)
pub fn get_data(file: Option<&str>, data: Option<&str>) -> String {
    if file.is_some() {
        println!("{:?}", file);
        read_to_string(file.unwrap()).expect("Error reading from file.")
    } else {
        data.unwrap().to_string()
    }
}

pub fn encrypt_data(cipher: &str, data: &str, key: &str) -> String {
    match cipher {
        "shift" => {
            let shift_amount = key.parse::<i8>();
            if shift_amount.is_err() {
                panic!("The key used in a shift cipher must be an integer (the amount by which to shift).");
            }
            shift::encrypt(data, shift_amount.unwrap())
        }
        "monoalphabetic" => {
            if !is_alphabet(key) {
                panic!("The key used in a monoalphabetic cipher must be a permutation of the English alphabet.");
            }
            monoalphabetic::encrypt(data, key)
        }
        "vigenere" => vigenere::encrypt(data, key),
        _ => {
            panic!("This cipher has not yet been implemented or it doesn't exist.");
        }
    }
}

pub fn decrypt_data(cipher: &str, data: &str, key: &str) -> String {
    match cipher {
        "shift" => {
            let shift_amount = key.parse::<i8>();
            if shift_amount.is_err() {
                panic!("The key used in a shift cipher must be an integer (the amount by which to shift).");
            }
            shift::decrypt(data, shift_amount.unwrap())
        }
        "monoalphabetic" => {
            if !is_alphabet(key) {
                panic!("The key used in a monoalphabetic cipher must be a permutation of the English alphabet.");
            }
            monoalphabetic::decrypt(data, key)
        }
        "vigenere" => vigenere::decrypt(data, key),
        _ => {
            panic!("This ciphers has not yet been implemented or it doesn't exist.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alphabet_detection() {
        // test a correct alphabet
        let possible_alphabet: String = ('a'..='z').collect();
        assert!(is_alphabet(&possible_alphabet));

        // test a correct but scrambled one
        let possible_alphabet = String::from("klmnopqrstuvwxyzabcdefghij");
        assert!(is_alphabet(&possible_alphabet));

        // test an obviously incorrect one
        let possible_alphabet = String::from("mbwrtoiu14576184tt9123485--");
        assert!(!is_alphabet(&possible_alphabet));

        // test a subtly incorrect one (missing an 'f', duplicate 'e')
        let possible_alphabet = String::from("klmnopqrstuvwxyzabcdeeghij");
        assert!(!is_alphabet(&possible_alphabet));

        // test another subtly incorrect one (the number '1' has been appended to a correct one)
        let possible_alphabet = String::from("klmnopqrstuvwxyzabcdefghij1");
        assert!(!is_alphabet(&possible_alphabet));
    }

    #[test]
    #[should_panic]
    fn test_nonexisting_file() {
        get_data(Some("nonexisting_file.txt"), Some("completely valid data"));
    }

    #[test]
    fn test_valid_data() {
        let inp = String::from("this is valid data");
        let outp = get_data(None, Some(&inp));

        assert_eq!(outp, inp);
    }
}
