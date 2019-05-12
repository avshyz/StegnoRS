#[macro_use]
extern crate clap;

use stegno::bits as bits;
use stegno::files as files;

use clap::{Arg, App, ArgGroup, SubCommand};

fn main() {
    let matches = App::new("stegno")
        .version("0.1.0")
        .author("Avishay 'avshyz' Zarad <zarad.avishay@gmail.com>")
        .about("Steganography toolkit written in Rust")
        .subcommand(
            SubCommand::with_name("read")
                .arg_from_usage("<input> 'input file'")
                .help("reads a file")
        )
        .subcommand(
            SubCommand::with_name("write")
                .arg_from_usage("<input> 'input file'")
                .arg_from_usage("-m, --message=<message>")
                .arg(Arg::from_usage("-o --output=[output]")
                    .default_value("out.bmp")
                )
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("read") => {
            let matches = matches.subcommand_matches("write").unwrap();
            let input = matches.value_of("input").unwrap();
        }
        Some("write") => {
            let matches = matches.subcommand_matches("write").unwrap();
            let input_file = matches.value_of("input").unwrap_or("xxx");
            let message = matches.value_of("message").unwrap_or("MMMM");
            let output = matches.value_of("output").unwrap_or("out");
            println!("{} {} {}", input_file, output, message)
        }
        _ => println!("Unrecognized command. Use -h or --help for usage")
    }
}