use clap::{App, Arg, SubCommand};
use regex::Regex;
use std::fs;
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
    let text = fs::read_to_string(target_file).unwrap();
    let lines = text.split("\n").collect::<Vec<_>>();

    let mut index_list: Vec<u32> = Vec::new();
    let re = Regex::new(&regex_pattrern).unwrap();
    for (i, line) in lines.iter().enumerate() {
        if re.is_match(line) {
            index_list.push(i.try_into().unwrap());
        }
    }
    index_list.push(lines.len().try_into().unwrap());

    let mut data_range: Vec<u32> = Vec::new();
    let mut i = 1;
    for index in index_list {
        let output = format!("./output_{}.txt", i);
        data_range.push(index);
        if data_range.len() == 2 {
            let first: u32 = *data_range.first().unwrap();
            let last: u32 = *data_range.last().unwrap();
            let result = &lines[first as usize..last as usize];
            let write_data = result.join("\n");

            let mut f = File::create(output).unwrap();
            let bytes = write_data.as_bytes();
            f.write_all(bytes).unwrap();

            let tmp_index: u32 = *data_range.last().unwrap();
            data_range.clear();
            data_range.push(tmp_index);
            i += 1;
        }
    }
    StatusData {
        status_code: 200,
        message: "File read successfully.".to_string(),
    }
}