use super::utils::clean_input;
use std::collections::VecDeque;

/// Shifts character ch by n in either direction
/// ==> (ch [+-] n) mod 26
pub fn shift_by(n: i8, ch: char) -> char {
    // don't encrypt digits
    if ch.is_ascii_digit() {
        return ch;
    }

    let mut alphabet = ('A'..='Z').collect::<VecDeque<char>>();
    let abs_shift = (n.abs() % 26) as u32;
    let idx = ch as u32 - 'A' as u32;

    if n < 0 {
        alphabet.rotate_right(abs_shift as usize);
    } else {
        alphabet.rotate_left(abs_shift as usize);
    }

    alphabet[idx as usize]
}

pub fn encrypt(plaintext: &str, key: i8) -> String {
    let clean_plaintext = clean_input(plaintext);

    clean_plaintext
        .chars()
        .map(|ch| shift_by(key, ch))
        .collect::<String>()
}

pub fn decrypt(ciphertext: &str, key: i8) -> String {
    let clean_ciphertext = clean_input(ciphertext);

    clean_ciphertext
        .chars()
        .map(|ch| shift_by(-key, ch))
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_by() {
        // basic tests
        assert_eq!('B', shift_by(1, 'A'));
        assert_eq!('C', shift_by(2, 'A'));
        assert_eq!('D', shift_by(3, 'A'));
        assert_eq!('B', shift_by(27, 'A'));

        // edge cases
        assert_eq!('A', shift_by(1, 'Z'));
        assert_eq!('Z', shift_by(-1, 'A'));
    }

    #[test]
    fn test_zero_shift() {
        let plaintext = String::from("Hello");

        assert_eq!(plaintext.to_uppercase(), encrypt(&plaintext, 0));
    }

    #[test]
    fn test_numeric_input() {
        let plaintext = String::from("123445678890");

        for shift in 1..=25 {
            assert_eq!(plaintext, encrypt(&plaintext, shift));
        }
    }

    #[test]
    fn test_known_pairs() {
        // from https://cryptii.com
        let plaintext = String::from("attackatdawn");
        let shift = 8;
        let ciphertext = String::from("ibbiksibliev");

        assert_eq!(ciphertext.to_uppercase(), encrypt(&plaintext, shift));

        let plaintext = String::from("firstman");
        let shift = 13;
        let ciphertext = String::from("svefgzna");

        assert_eq!(ciphertext.to_uppercase(), encrypt(&plaintext, shift));
    }

    #[test]
    fn test_correct() {
        let plaintext = String::from("caesar");

        // it's actually just enough to check from 1 to 25, but just to be sure...
        for shift in 0..33 {
            assert_eq!(
                plaintext.to_uppercase(),
                decrypt(&encrypt(&plaintext, shift), shift)
            );
        }
    }
}
