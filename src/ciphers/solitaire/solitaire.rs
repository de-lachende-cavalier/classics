use super::card_deck::Deck;
// encryption/decryption functions just like Vigenere => repeated shift cipher
use super::super::shift::ShiftCipher;
use crate::Cipher;

pub struct SolitaireCipher {
    key: String,
}

impl SolitaireCipher {
    pub fn new(key: &str) -> Self {
        SolitaireCipher {
            key: key.to_string(),
        }
    }
}

impl Cipher for SolitaireCipher {
    fn encrypt(&self, plaintext: &str) -> String {
        let clean_plaintext = <SolitaireCipher as Cipher>::clean_input(plaintext);
        let mut ciphertext: String = String::new();
        let mut deck = Deck::new();

        // only key deck once!
        deck.key_deck(&self.key);

        for ch in clean_plaintext.chars() {
            let stream_value = deck.get_output_card();
            let enc_char = ShiftCipher::shift_by(stream_value as i8, ch);

            ciphertext.push(enc_char);
        }

        ciphertext
    }

    fn decrypt(&self, ciphertext: &str) -> String {
        let clean_ciphertext = <SolitaireCipher as Cipher>::clean_input(ciphertext);
        let mut plaintext: String = String::new();
        let mut deck = Deck::new();

        // only key deck once!
        deck.key_deck(&self.key);

        for ch in clean_ciphertext.chars() {
            let stream_value = deck.get_output_card();
            // note the minus sign
            let dec_char = ShiftCipher::shift_by(-(stream_value as i8), ch);

            plaintext.push(dec_char);
        }

        plaintext
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;

    #[test]
    #[ignore]
    fn test_correct() {
        let keys = vec!["lkajhfd", "lslsl s asd", "lk5y&/_7t274otg", "@#@@[^^∏ß"];
        let plaintext = String::from("cardgames");

        for _ in 0..100 {
            let choice = keys.choose(&mut rand::thread_rng()).unwrap();

            let cipher = SolitaireCipher::new(*choice);
            for _ in 0..10 {
                assert_eq!(
                    plaintext.to_uppercase(),
                    cipher.decrypt(&cipher.encrypt(&plaintext))
                );
            }
        }
    }
}
