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
            let matches = matches.subcommand_matches("read").unwrap();
            let input_path = matches.value_of("input").unwrap();
            // TODO - add error handling
            // TODO - maybe encapsulate better
            let file_data = files::bmp::read(input_path).unwrap();
            let extracted = files::bmp::extract(&file_data).unwrap();
            println!("{}", extracted);
        }
        Some("write") => {
            let matches = matches.subcommand_matches("write").unwrap();
            let input_path = matches.value_of("input").unwrap();
            let message = matches.value_of("message").unwrap();
            let output = matches.value_of("output").unwrap();

            let file_data = files::bmp::read(input_path).unwrap();
            let injected = files::bmp::inject(&file_data, message).unwrap();
            files::bmp::write(output, &injected).unwrap();
        }
        _ => println!("Unrecognized command. Use -h or --help for usage")
    }
}