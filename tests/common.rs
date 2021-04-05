use std::io::{Error, Write};
use std::process::{Command, Output};

use std::fs;

pub fn setup() -> String {
    let mut f = fs::File::create("tests/testing.txt").expect("Error creating file testing.txt.");
    let text = " 
This text serves in my integration testing
it's basically a prototype for a possible file and its contents.

Super secret code: jjho45gab78234boiua97345b2l34572345jgh243587t!
Don't share with anyone!!
";

    f.write_all(text.as_bytes())
        .expect("Error writing to testing.txt.");

    text.to_string()
}

pub fn teardown() {
    fs::remove_file("tests/testing.txt").expect("Error deleting file testing.txt");
}

pub fn run_with_args(args: &Vec<&str>) -> Result<Output, Error> {
    Command::new("target/debug/classical_cryptography")
        .args(args)
        .output()
}
