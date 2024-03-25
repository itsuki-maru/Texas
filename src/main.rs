use clap::{App, Arg, SubCommand};

mod scheme;
mod rustex;
use rustex::split_file;

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
    
    // "split" ex) rustex split -t ./testfile/test1.txt -r "^第[1-9]章"
    if let Some(matches) = matches.subcommand_matches("split") {
        if let (Some(target_file), Some(regex_pattrern)) = (matches.value_of("target"), matches.value_of("regex")) {
            let result = split_file(target_file, regex_pattrern);
            println!("Status Code: {}, Message: {}", result.status_code, result.message);
            return;
        }
    }
}