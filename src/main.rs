use clap::{App, Arg, SubCommand};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write, Read};
use std::path::Path;


fn main() {
    let matches = App::new("rustex")
        .version("1.0")
        .author("Itsuki Maru")
        .about("Text file proessing tool.")
        .subcommand(
            SubCommand::with_name("split")
                .about("Splits a file based on a reguler expression.")
                .arg(Arg::with_name("target")
                    .short('t')
                    .long("target")
                    .value_name("FILE")
                    .help("Target text file (csv, txt)")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("regex")
                .short('r')
                .long("regex")
                .value_name("REGEX")
                .help("Reguler expression for splitting.")
                .required(true)
                .takes_value(true))
        ).get_matches();
    
    if let Some(matches) = matches.subcommand_matches("split") {
        if let (Some(target_file), Some(regex_pattrern)) = (matches.value_of("target"), matches.value_of("regex")) {
            let result = split_file(target_file, regex_pattrern);
            println!("Status Code: {}, Message: {}", result.status_code, result.message);
        }
    }
}

struct StatusData {
    status_code: u32,
    message: String,
}

fn split_file(target_file: &str, regex_pattrern: &str) -> StatusData {
    // let text = fs::read_to_string(target_file).unwrap();
    let mut file = match File::open(target_file) {
        Ok(file) => file,
        Err(_) => return StatusData {
            status_code: 400,
            message: "File not found.".to_string(),
        },
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => StatusData {
            status_code: 200,
            message: "File read successfully.".to_string(),
        },
        Err(e) => StatusData {
            status_code: 500,
            message: format!("Error reading file: {}", e),
        },
    }
}