// public, accessible by the whole crate
pub mod monoalphabetic;
pub mod scytale;
pub mod shift;
pub mod solitaire;
pub mod vigenere;

// accessible only by modules within ciphers/
mod card_deck;
mod utils;
