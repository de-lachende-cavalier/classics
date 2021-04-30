// Vigenere is just a repeated shift cipher
use super::shift::ShiftCipher;
use crate::Cipher;

pub struct VigenereCipher {
    key: String,
}

impl VigenereCipher {
    pub fn new(key: &str) -> Self {
        VigenereCipher {
            key: key.to_string(),
        }
    }
}

impl Cipher for VigenereCipher {
    fn encrypt(&self, plaintext: &str) -> String {
        let clean_plaintext = <VigenereCipher as Cipher>::clean_input(plaintext);

        let upper_key = self.key.to_uppercase();
        let mut ciphertext: Vec<char> = Vec::new();

        for (idx, ch) in clean_plaintext.chars().enumerate() {
            let ch_k = upper_key.as_bytes()[(idx % self.key.len())];
            let shift = (ch_k as u32 - 'A' as u32) as i8;

            ciphertext.push(ShiftCipher::shift_by(shift, ch));
        }

        ciphertext.iter().collect::<String>()
    }

    fn decrypt(&self, ciphertext: &str) -> String {
        let clean_ciphertext = <VigenereCipher as Cipher>::clean_input(ciphertext);

        let upper_key = self.key.to_uppercase();
        let mut plaintext: Vec<char> = Vec::new();

        for (idx, ch) in clean_ciphertext.chars().enumerate() {
            let ch_k = upper_key.as_bytes()[(idx % self.key.len())];
            let shift = (ch_k as u32 - 'A' as u32) as i8;

            plaintext.push(ShiftCipher::shift_by(-shift, ch));
        }

        plaintext.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;

    #[test]
    #[should_panic]
    fn test_empty_key() {
        let cipher = VigenereCipher::new("");

        cipher.encrypt("Hello");
    }

    #[test]
    fn test_key_longer_than_pt() {
        let cipher = VigenereCipher::new("testinglongkey");
        let ciphertext = String::from("llgkbgkih");

        assert_eq!(ciphertext.to_uppercase(), cipher.encrypt("shorttext"));
    }

    #[test]
    fn test_known_pairs() {
        // from https://en.wikipedia.org/wiki/Vigenere_cipher
        let cipher = VigenereCipher::new("lemon");
        let plaintext = "attackatdawn";
        let ciphertext = String::from("lxfopvefrnhr");

        assert_eq!(ciphertext.to_uppercase(), cipher.encrypt(plaintext));

        assert_eq!(
            <VigenereCipher as Cipher>::clean_input(plaintext),
            cipher.decrypt(&ciphertext)
        );

        // from https://cryptii.com/
        let cipher = VigenereCipher::new("cryptii");
        let plaintext = "firstman";
        let ciphertext = String::from("hzphmuip");

        assert_eq!(ciphertext.to_uppercase(), cipher.encrypt(plaintext));

        assert_eq!(
            <VigenereCipher as Cipher>::clean_input(plaintext),
            cipher.decrypt(&ciphertext)
        );
    }

    #[test]
    #[ignore]
    fn test_correct() {
        let keys = [
            "lemon",
            "jackpot",
            "nu",
            "anincrediblylongkeyaverylongkeyindeed",
        ];
        let plaintext = String::from("lechiffre");

        for _ in 0..1000 {
            let choice = keys.choose(&mut rand::thread_rng()).unwrap();

            let cipher = VigenereCipher::new(*choice);
            assert_eq!(
                plaintext.to_uppercase(),
                cipher.decrypt(&cipher.encrypt(&plaintext))
            );
        }
    }
}
