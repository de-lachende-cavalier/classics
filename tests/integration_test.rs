mod common;
use common::*;

use classical_cryptography::encrypt_data;

use std::str::from_utf8;

#[test]
fn test_reading_from_stdin() {
    let cipher = "vigenere";
    let plaintext = "Super secret password: JLKIIn4937774800984hl98457";
    let key = "cantguessthisone";

    let out = run_with_args(&vec![cipher, key, plaintext, "-e"]).expect("Error running the binary");
    assert!(out.status.success());

    let mut str_out = from_utf8(&out.stdout)
        .expect("Error reading stdout")
        .to_string();
    assert!(str_out.contains("Encrypted data:"));

    str_out = str_out.replace("\n", "");
    let enc_out = str_out.split(":").collect::<Vec<&str>>()[1].to_string();

    let encrypted = encrypt_data(cipher, plaintext, key);
    assert_eq!(enc_out, encrypted);
}

#[test]
fn test_reading_from_file() {
    let data = setup();
    let cipher = "monoalphabetic";
    let key = "yzxwvutsrqmonplkjihgfadcbe";

    let out = run_with_args(&vec![cipher, key, "-e", "-f", "tests/testing.txt"])
        .expect("Error running the binary.");
    println!("{:?}", out);
    assert!(out.status.success());

    let mut str_out = from_utf8(&out.stdout)
        .expect("Error reading stdout.")
        .to_string();
    assert!(str_out.contains("Encrypted data:"));

    str_out = str_out.replace("\n", "");
    let enc_out = str_out.split(":").collect::<Vec<&str>>()[1].to_string();

    let encrypted = encrypt_data(cipher, &data, key);
    assert_eq!(enc_out, encrypted);

    teardown();
}
