use clap::{App, Arg};
use std::fs;
use std::io::{self, Read};
use std::path::Path;

// Configuration struct to hold parser settings
struct Config {
    input_file: String,
    verbose: bool,
}

// CLI parser function
fn parse_cli() -> Config {
    let matches = App::new("fyvx_construct_a_sec")
        .version("1.0")
        .author("Your Name")
        .about("A secure CLI tool parser")
        .arg(
            Arg::with_name("input")
                .long("input")
                .short("i")
                .takes_value(true)
                .required(true)
                .help("Input file to parse"),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .takes_value(false)
                .help("Enable verbose mode"),
        )
        .get_matches();

    let input_file = matches.value_of("input").unwrap().to_string();
    let verbose = matches.is_present("verbose");

    Config { input_file, verbose }
}

// Secure file reader function
fn read_file_securely<P>(path: P) -> Result<String, io::Error>
where
    P: AsRef<Path>,
{
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let config = parse_cli();
    let input_contents = read_file_securely(config.input_file).unwrap_or("Error reading file!".to_string());

    if config.verbose {
        println!("Verbose mode enabled");
    }

    println!("Input file contents: {}", input_contents);
}