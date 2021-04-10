use super::utils::{clean_input, pad};

fn get_diameter(s: &str, length: usize) -> usize {
    let padded_s = pad(s, length as u32);

    padded_s.len() / length

}

fn wrap_around_scytale(text: &str, n: usize) -> String {
    let mut unwinded = String::new();
    let l = text.len();

    for i in 0..=l {
        for j in (i..l).step_by(n) {
            unwinded.push(text.chars().nth(j).unwrap());
        }
    }
    // TODO can i get the proper length in the loops above
    // without having to truncate?
    unwinded.truncate(l);

    unwinded
}

pub fn encrypt(plaintext: &str, length: usize) -> String {
    let clean_plaintext = clean_input(plaintext);

    wrap_around_scytale(&clean_plaintext, length)
}

pub fn decrypt(ciphertext: &str, length: usize) -> String {
    let clean_ciphertext = clean_input(ciphertext);
    let diameter = get_diameter(&clean_ciphertext, length);

    wrap_around_scytale(&clean_ciphertext, diameter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_zero_length() {
        encrypt("this should fail", 0);
    }

    #[test]
    fn test_known_pairs() {
        // from https://en.wikipedia.org/wiki/Scytale
        let plaintext = "I am hurt very badly help";
        let length = 5;
        let ciphertext = String::from("Iryyatbhmvaehedlurlp");

        assert_eq!(ciphertext.to_uppercase(), encrypt(plaintext, length));

        assert_eq!(clean_input(plaintext), decrypt(&ciphertext, length));

        // from 'Programming in Cryptol', page 30
        let plaintext = "attackatdawn";
        let length = 4; // in the book the diameter is 3
        let ciphertext = String::from("ACDTKATAWATN");

        assert_eq!(ciphertext, encrypt(plaintext, length));

        assert_eq!(clean_input(plaintext), decrypt(&ciphertext, length));
    }

    #[test]
    fn test_correct() {
        let plaintext = String::from("Iamhurtverybadly");

        // FIXME something very subtle is going on here...
        for length in 1..=plaintext.len() {
                println!("{}", length);
                assert_eq!(
                    plaintext.to_uppercase(),
                    decrypt(&encrypt(&plaintext, length), length)
                );
        }
    }
}
