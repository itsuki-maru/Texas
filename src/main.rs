use clap::{App, Arg, SubCommand};

mod scheme;
mod rustex;
use rustex::split_file;
use std::env;

fn main() {
    // カレントディレクトリを取得
    let current_directory = env::current_dir().expect("Current Directoru Get Error.");
    let current_dir_str = current_directory.to_str().unwrap_or("Invalied Path.");

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
                .arg(Arg::with_name("output")
                    .short('o')
                    .long("output")
                    .value_name("OUTPUT")
                    .help("Output directory.")
                    .required(false)
                    .takes_value(true)
                    .default_value(current_dir_str))
        ).get_matches();
    
    // "split" ex) rustex split -t ./testfile/test1.txt -r "^第[1-9]章"
    if let Some(matches) = matches.subcommand_matches("split") {
        if let (Some(target_file), Some(regex_pattrern), Some(output_dir)) = (matches.value_of("target"), matches.value_of("regex"), matches.value_of("output")) {
            let result = split_file(target_file, regex_pattrern, output_dir);
            println!("Status Code: {}, Message: {}", result.status_code, result.message);
            return;
        }
    }
}