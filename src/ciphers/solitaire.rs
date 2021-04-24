use super::card_deck::Deck;
// encryption/decryption functions just like Vigenere => repeated shift cipher
use super::shift::ShiftCipher;
use super::utils::pad;
use crate::Cipher;

pub struct SolitaireCipher {
    keystream: Deck,
}

impl SolitaireCipher {
    pub fn new(key: &str) -> Self {
        let mut deck = Deck::new();
        deck.prepare(key);

        SolitaireCipher { keystream: deck }
    }
}

impl Cipher for SolitaireCipher {
    fn encrypt(&self, plaintext: &str) -> String {
        todo!();
    }
    fn decrypt(&self, ciphertext: &str) -> String {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
