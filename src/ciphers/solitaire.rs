use super::card_deck::Deck;
// encryption/decryption functions just like Vigenere => repeated shift cipher
use super::shift::ShiftCipher; 
use super::utils::pad;
use crate::Cipher;

pub struct SolitaireCipher {
    key: Deck, // the keystream, aka the Deck of cards
}

impl SolitaireCipher {}

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
