use crate::Cipher;
use std::collections::VecDeque;

pub struct ShiftCipher {
    key: i8,
}

impl ShiftCipher {
    pub fn new(key: i8) -> Self {
        ShiftCipher { key }
    }

    /// Shifts character ch by n in either direction
    /// ==> (ch [+-] n) mod 26
    pub fn shift_by(n: i8, ch: char) -> char {
        // don't encrypt digits
        if ch.is_ascii_digit() {
            return ch;
        }

        // VecDeque is faster than Vec at rotations
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
}

impl Cipher for ShiftCipher {
    fn encrypt(&self, plaintext: &str) -> String {
        let clean_plaintext = <ShiftCipher as Cipher>::clean_input(plaintext);

        clean_plaintext
            .chars()
            .map(|ch| ShiftCipher::shift_by(self.key, ch))
            .collect::<String>()
    }

    fn decrypt(&self, ciphertext: &str) -> String {
        let clean_ciphertext = <ShiftCipher as Cipher>::clean_input(ciphertext);

        clean_ciphertext
            .chars()
            .map(|ch| ShiftCipher::shift_by(-self.key, ch))
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_by() {
        // basic tests
        assert_eq!('B', ShiftCipher::shift_by(1, 'A'));
        assert_eq!('C', ShiftCipher::shift_by(2, 'A'));
        assert_eq!('D', ShiftCipher::shift_by(3, 'A'));
        assert_eq!('B', ShiftCipher::shift_by(27, 'A'));

        // edge cases
        assert_eq!('A', ShiftCipher::shift_by(1, 'Z'));
        assert_eq!('Z', ShiftCipher::shift_by(-1, 'A'));
    }

    #[test]
    fn test_zero_shift() {
        let cipher = ShiftCipher::new(0);
        let plaintext = String::from("Hello");

        assert_eq!(plaintext.to_uppercase(), cipher.encrypt(&plaintext));
    }

    #[test]
    fn test_numeric_input() {
        for shift in 1..=25 {
            let cipher = ShiftCipher::new(shift);
            let plaintext = "918273456";

            assert_eq!(plaintext.to_string(), cipher.encrypt(plaintext));
        }
    }

    #[test]
    fn test_known_pairs() {
        // from https://cryptii.com
        let cipher = ShiftCipher::new(8);
        let ciphertext = String::from("ibbiksibliev");

        assert_eq!(ciphertext.to_uppercase(), cipher.encrypt("attackatdawn"));

        let cipher = ShiftCipher::new(13);
        let ciphertext = String::from("svefgzna");

        assert_eq!(ciphertext.to_uppercase(), cipher.encrypt("firstman"));
    }

    #[test]
    #[ignore]
    fn test_correct() {
        let plaintext = String::from("caesar");
        
        // it's actually just enough to check for shifts from 1 to 25...
        for shift in 0..=127 {
            let cipher = ShiftCipher::new(shift);

            assert_eq!(
                plaintext.to_uppercase(),
                cipher.decrypt(&cipher.encrypt(&plaintext))
            );
        }
    }
}
