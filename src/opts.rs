/// This module is meant to store anything relating to the managment of CLI opts (/args)
extern crate clap;
use clap::{App, Arg, ArgMatches};

pub fn get_opts() -> ArgMatches<'static> {
    App::new("-- Classical Crypto Toolkit --")
        .about("A collection of classical ciphers.")
        .help_message("Prints help information (--help for more).")
        .version_message("Prints version information.")
        .arg(
            Arg::with_name("cipher")
                .help("Specifies the cipher to use (use --help to see a complete list).")
                // TODO put all these options in a config file
                .long_help("Available options are: shift, monoalphabetic, scytale and vigenere.")
                .required(true)
                .takes_value(true)
                .index(1),
        )
        .arg(
            Arg::with_name("key")
                .help("Specifies the key used to decrypt/encrypt.")
                .required(true)
                .takes_value(true)
                .index(2),
        )
        .arg(Arg::with_name("data")
                .help("Specifies the data to read from stdin (in case no file has been specified with -f.)")
                .takes_value(true)
                .required_unless("file")
                .index(3)
            )
        .arg(
            Arg::with_name("encrypt")
                .short("e")
                .long("encrypt")
                .help(
                    "Instructs the binary to do encryption. If unspecified, the -d option must be.",
                )
                .required_unless("decrypt"),
        )
        .arg(
            Arg::with_name("decrypt")
                .short("d")
                .long("decrypt")
                .help(
                    "Instructs the binary to do decryption. If unspecified, the -e option must be.",
                )
                .required_unless("encrypt"),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Specifies a file to read from. If not specified, stdin is used.")
                .takes_value(true)
                .required_unless("data"),
        )
        .get_matches()
}
