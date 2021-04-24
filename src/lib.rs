mod ciphers;

use ciphers::solitaire::solitaire::SolitaireCipher;
use ciphers::monoalphabetic::MonoalphaCipher;
use ciphers::vigenere::VigenereCipher;
use ciphers::shift::ShiftCipher;
use ciphers::scytale::Scytale;

use std::fs::read_to_string;

pub(crate) trait Cipher {
    /// Cleans up the input by removing all characters that are not alphanumeric
    /// (Returns an uppercase String)
    fn clean_input(input: &str) -> String {
        let cleaned = input
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect::<String>();

        cleaned
            .split_whitespace()
            .collect::<String>()
            .to_uppercase()
    }

    fn encrypt(&self, plaintext: &str) -> String;
    fn decrypt(&self, ciphertext: &str) -> String;
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
        // TODO put all these options in a config file
        "shift" => {
            let shift_amount = key.parse::<i8>();
            if shift_amount.is_err() {
                panic!("The key used in a shift cipher must be an integer (the amount by which to shift).");
            }

            let sc = ShiftCipher::new(shift_amount.unwrap());

            sc.encrypt(data)
        }
        "monoalphabetic" => {
            let mc = MonoalphaCipher::new(key);

            mc.encrypt(data)
        }
        "vigenere" => {
            let vc = VigenereCipher::new(key);

            vc.encrypt(data)
        }
        "scytale" => {
            let length = key.parse::<usize>();
            if length.is_err() {
                panic!("The key used for a scytale cipher is the length of the scytale itself, so must be a uint.");
            }

            let scytale = Scytale::new(length.unwrap());

            scytale.encrypt(data)
        }
        "solitaire" => {
            let solitaire_c = SolitaireCipher::new(key);

            solitaire_c.encrypt(data)
        }
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

            let sc = ShiftCipher::new(shift_amount.unwrap());

            sc.decrypt(data)
        }
        "monoalphabetic" => {
            let mc = MonoalphaCipher::new(key);

            mc.decrypt(data)
        }
        "vigenere" => {
            let vc = VigenereCipher::new(key);

            vc.decrypt(data)
        }
        "scytale" => {
            let length = key.parse::<usize>();
            if length.is_err() {
                panic!("The key used for a scytale cipher is the length of the scytale itself, so must be a uint.");
            }

            let scytale = Scytale::new(length.unwrap());

            scytale.decrypt(data)
        }
        "solitaire" => {
            let solitaire_c = SolitaireCipher::new(key);

            solitaire_c.decrypt(data)
        }
        _ => {
            panic!("This cipher has not yet been implemented or it doesn't exist.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_input() {
        struct T {}

        impl Cipher for T {
            fn encrypt(&self, _plaintext: &str) -> String {
                String::new()
            }
            fn decrypt(&self, _ciphertext: &str) -> String {
                String::new()
            }
        }

        let input = String::from("awesome_testing_functionality with spaces");
        let expected = String::from("AWESOMETESTINGFUNCTIONALITYWITHSPACES");
        assert_eq!(expected, T::clean_input(&input));

        let input = String::from("NoW@wITHéé˛Ånumb3rz00712");
        let expected = String::from("NOWWITHNUMB3RZ00712");
        assert_eq!(expected, T::clean_input(&input));

        assert_eq!("".to_string(), T::clean_input(""));
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
