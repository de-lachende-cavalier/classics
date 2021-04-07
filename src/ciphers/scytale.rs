use super::utils::clean_input;

// TODO make the scytale into a struct to keep data about it
// TODO (like the number of rows and columns)
fn build_scytale(pt: &str, diameter: u8) -> Vec<Vec<char>> {
    // attack at dawn
    // diameter 3
    // => a t t
    //    a c k
    //    a t d
    //    a w n
    // lucky for me, Rust is awesome and it has chunks()!
    let v_chars = pt.chars().collect::<Vec<char>>();
    let chunks = v_chars.chunks(diameter as usize).collect::<Vec<_>>();

    chunks
        .iter()
        .map(|v| v.to_vec())
        .collect::<Vec<Vec<char>>>()
}

pub fn encrypt(plaintext: &str, key: u8) -> String {
    let clean_plaintext = clean_input(plaintext);
    let scytale = build_scytale(&clean_plaintext, key);
    let nrows = scytale.len();
    println!("{:?}", scytale);

    let mut ciphertext = String::new();

    // FIXME this fails for a very obvious reason (look at the output of println)
    for i in 0..key as usize {
        for j in 0..nrows {
            ciphertext.push(scytale[j][i]);
        }
    }

    ciphertext
}

// => the key provided doesn't work for decryption!! => the scytale matrix is not
// the same => it's transposed of the one used for encryption 
// => num of rows = num of columns!
pub fn decrypt(ciphertext: &str, key: u8) -> String {
    let clean_ciphertext = clean_input(ciphertext);
    // this line should be modified
    // the lines below are a pretty ugly hack 
    // TODO fix these (best way is probably making a struct as above)
    let scytale = build_scytale(&clean_ciphertext, key);
    let nrows = scytale.len();
    let scytale = build_scytale(&clean_ciphertext, nrows as u8);

    let mut plaintext = String::new();

    for i in 0..nrows {
        for j in 0..key as usize {
            plaintext.push(scytale[j][i]);
        }
    }

    plaintext
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_zero_diameter() {
        build_scytale("this should not work", 0);
    }

    #[test]
    fn test_build_scytale() {
        // from https://en.wikipedia.org/wiki/Scytale
        let clean_plaintext = clean_input("I am hurt very badly help");
        let key = 5;
        let expected_scytale: Vec<Vec<char>> = vec![
            ['I', 'A', 'M', 'H', 'U'].to_vec(),
            ['R', 'T', 'V', 'E', 'R'].to_vec(),
            ['Y', 'B', 'A', 'D', 'L'].to_vec(),
            ['Y', 'H', 'E', 'L', 'P'].to_vec(),
        ];
        let scytale = build_scytale(&clean_plaintext, key);

        assert_eq!(scytale, expected_scytale);

        // edge cases
        let clean_plaintext = clean_input("testing edge");
        let key = 1;
        let expected_scytale: Vec<Vec<char>> = vec![
            ['T'].to_vec(),
            ['E'].to_vec(),
            ['S'].to_vec(),
            ['T'].to_vec(),
            ['I'].to_vec(),
            ['N'].to_vec(),
            ['G'].to_vec(),
            ['E'].to_vec(),
            ['D'].to_vec(),
            ['G'].to_vec(),
            ['E'].to_vec(),
        ];
        let scytale = build_scytale(&clean_plaintext, key);

        assert_eq!(scytale, expected_scytale);

        let clean_plaintext = clean_input("short pt");
        let key = 100;
        let expected_scytale: Vec<Vec<char>> = vec![
            ['S', 'H', 'O', 'R', 'T', 'P', 'T'].to_vec(),
        ];
        let scytale = build_scytale(&clean_plaintext, key);

        assert_eq!(scytale, expected_scytale);
    }

    #[test]
    fn test_known_pairs() {
        // from https://en.wikipedia.org/wiki/Scytale
        let plaintext = "I am hurt very badly help";
        let key = 5;
        let ciphertext = String::from("Iryyatbhmvaehedlurlp");

        assert_eq!(ciphertext.to_uppercase(), encrypt(plaintext, key));
        assert_eq!(clean_input(plaintext), decrypt(&ciphertext, key));
        
    }

    #[test]
    fn test_correct() {
        let plaintext = "for sparta, dude!";

        for diameter in 1..=plaintext.len() {
            assert_eq!(
                clean_input(plaintext),
                decrypt(&encrypt(plaintext, diameter as u8), diameter as u8)
            );
        }
    }
}
