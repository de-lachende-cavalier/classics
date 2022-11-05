/// This crate implements a bunch of classical ciphers.
mod opts;

use classical_cryptography::{decrypt_data, encrypt_data, get_data};
use opts::get_opts;

fn main() {
    let opts = get_opts();

    let cipher = opts.value_of("cipher").unwrap();
    let key = opts.value_of("key").unwrap();

    let data = get_data(opts.value_of("file"), opts.value_of("data"));

    // no fancy stuff output-wise to allow people to easily edit the output when
    // redirecting to other files/using pipes
    if opts.is_present("encrypt") {
        let encrypted = encrypt_data(cipher, &data, key);
        println!("Encrypted data:\n\n{}\n", encrypted);
    } else if opts.is_present("decrypt") {
        let decrypted = decrypt_data(cipher, &data, key);
        println!("Decrypted data:\n\n{}\n", decrypted);
    } else {
        panic!("You have to specify whether you want to decrypt or encrypt!");
    }
}
