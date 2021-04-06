// TODO clean up this mess!!
use super::utils::clean_input;

fn build_scytale(pt: &str, diameter: u8) -> Vec<&[char]> {
    // attack at dawn
    // diameter 3
    // => a t t
    //    a c k
    //    a t d
    //    a w n
    // lucky for me, Rust is awesome and it has chunks()!
    unimplemented!(); // not true...
    // let chars = pt.chars().collect::<Vec<char>>();
    // chars.chunks(diameter as usize).collect::<Vec<_>>()
}

pub fn encrypt(plaintext: &str, key: u8) -> String {
    let clean_plaintext = clean_input(plaintext);

    let chars = clean_plaintext.chars().collect::<Vec<char>>();
    let scytale = chars.chunks(key as usize).collect::<Vec<_>>();
    let mut ciphertext = String::new();

    for i in 0..key as usize {
        let nrows = scytale.len();
        for j in 0..nrows {
            ciphertext.push(scytale[j][i]);
        }
    }

    ciphertext
}

pub fn decrypt(ciphertext: &str, key: u8) -> String {
    let clean_ciphertext = clean_input(ciphertext);
    unimplemented!();
}

#[cfg(test)]
mod tests {
    // TODO write the tests!!
    use super::*;

    #[test]
    fn test_wiki_input() {
        let plaintext = "I am hurt very badly help";
        let key = 5;
        let ciphertext = String::from("Iryyatbhmvaehedlurlp");

        assert_eq!(ciphertext.to_uppercase(), encrypt(plaintext, key));
    }

}

