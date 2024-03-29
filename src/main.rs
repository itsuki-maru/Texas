use clap::{App, Arg, Command};
use std::collections::HashSet;
use std::env;
mod rustex;
mod utils;
use rustex::{
    aggregate::aggregate_csv_data,
    groupby::groupby_column_csv,
    head::{print_head, print_header_csv},
    sortcsv::sort_csv_by_column, split::split_file,
    excol::extract_column,
    clean::clean_row,
    collect::collect_file,
    grep::grep_row,
    blocksplit::block_split,
    red::red,
    sum::sum,
    csvtojson::csv_to_json,
    lastrow::get_last_row,
    wc::{line_count, word_count},
};

fn main() {
    // カレントディレクトリを取得
    let current_directory = env::current_dir().expect("Current Directoru Get Error.");
    let current_dir_str = current_directory.to_str().unwrap_or("Invalied Path.");

    let matches = App::new("rustex")
        .version("1.0")
        .author("Itsuki Maru")
        .about("Text file proessing tool.")
        .subcommand(
            Command::new("split")
                .about("Splits a file based on a reguler expression.")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("FILE")
                        .help("Target text file (csv, txt)")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("regex")
                        .short('r')
                        .long("regex")
                        .value_name("REGEX")
                        .help("Reguler expression for splitting.")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("Output directory.")
                        .required(false)
                        .takes_value(true)
                        .default_value(current_dir_str),
                ),
        )
        .subcommand(
            Command::new("sortcsv")
                .about("Sort CSV.")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("FILE")
                        .help("Target text file (csv, txt)")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("colname")
                        .short('c')
                        .long("column")
                        .value_name("COLUMN")
                        .help("CSV sorted by column.")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("reverse")
                        .short('r')
                        .long("reverse")
                        .value_name("REVERSE")
                        .help("Reverse CSV sort.")
                        .required(false)
                        .takes_value(false)
                ),
        )
        .subcommand(
            Command::new("groupby")
                .about("Groupby CSV column.")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("FILE")
                        .help("Target text file (csv, txt)")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("colname")
                        .short('c')
                        .long("column")
                        .value_name("COLUMN")
                        .help("CSV groupby column.")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            Command::new("aggregate")
                .about("Aggregate CSV.")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("FILE")
                        .help("Target text file (csv, txt)")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("keycol")
                        .short('k')
                        .long("keycolumn")
                        .value_name("COLUMN")
                        .help("Key column.")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("columns")
                        .short('c')
                        .long("columns")
                        .value_name("TARGET COLUMNS")
                        .help("Target columns.")
                        .required(true)
                        .takes_value(true)
                        .multiple(true)
                )
                .arg(
                    Arg::new("floatmode")
                        .short('f')
                        .long("floatmode")
                        .value_name("FLOAT MODE")
                        .help("Aggregate float.")
                        .required(false)
                        .takes_value(false)
                )
                .arg(
                    Arg::new("iscsv")
                        .short('i')
                        .long("iscsv")
                        .value_name("OUTPUT CSV")
                        .help("Std Output To CSV.")
                        .required(false)
                        .takes_value(false)
                ),
        )
        .subcommand(
            Command::new("head")
                .about("Print header.")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("FILE")
                        .help("Target text file (csv, txt)")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("iscsv")
                        .short('c')
                        .long("iscsv")
                        .value_name("CSV CODE")
                        .help("CSV header row print.")
                        .required(false)
                        .takes_value(false)
                )
                .arg(
                    Arg::new("limit")
                        .short('l')
                        .long("limit")
                        .value_name("FILE")
                        .help("Read row limit.")
                        .required(false)
                        .takes_value(true)
                        .value_parser(clap::value_parser!(usize)),
                ),
        )
        .subcommand(
            Command::new("excol")
                .about("Extract column CSV.")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("FILE")
                        .help("Target text file (csv)")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("columns")
                        .short('c')
                        .long("columns")
                        .value_name("TARGET COLUMNS")
                        .help("Target columns.")
                        .required(true)
                        .takes_value(true)
                        .multiple(true)
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("Output directory.")
                        .required(false)
                        .takes_value(true)
                        .default_value(current_dir_str),
                ),
        )
        .subcommand(
            Command::new("clean")
                .about("Delete a file line based on a reguler expression.")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("FILE")
                        .help("Target text file (csv, txt)")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("regex")
                        .short('r')
                        .long("regex")
                        .value_name("REGEX")
                        .help("Reguler expression for splitting.")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("Output directory.")
                        .required(false)
                        .takes_value(true)
                        .default_value(current_dir_str),
                ),
        )
        .subcommand(
            Command::new("collect")
                .about("Collect file on a reguler expression.")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("FILE")
                        .help("Target text file (csv, txt)")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("Output directory.")
                        .required(false)
                        .takes_value(true)
                        .default_value(current_dir_str),
                )
                .arg(
                    Arg::new("regex")
                        .short('r')
                        .long("regex")
                        .value_name("REGEX")
                        .help("Reguler expression for splitting.")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            Command::new("grep")
                .about("Grep file on a reguler expression.")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("FILE")
                        .help("Target text file (csv, txt)")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("regex")
                        .short('r')
                        .long("regex")
                        .value_name("REGEX")
                        .help("Reguler expression for splitting.")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("Output directory.")
                        .required(false)
                        .takes_value(true)
                        .default_value(current_dir_str),
                )
                .arg(
                    Arg::new("csv")
                        .short('c')
                        .long("csv")
                        .value_name("CSV HEADER")
                        .help("CSV header row insert.")
                        .required(false)
                        .takes_value(false)
                ),
        )
        .subcommand(
            Command::new("blocksplit")
                .about("Sorted CSV Block Split.")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("FILE")
                        .help("Target text file (csv, txt)")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("colname")
                        .short('c')
                        .long("column")
                        .value_name("COLUMN")
                        .help("CSV split by column.")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("Output directory.")
                        .required(false)
                        .takes_value(true)
                        .default_value(current_dir_str),
                ),
        )
        .subcommand(
            Command::new("red")
                .about("Replaced text reguler expression.")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("FILE")
                        .help("Target text file (csv, txt)")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("regex")
                        .short('r')
                        .long("regex")
                        .value_name("REGEX")
                        .help("Reguler expression for splitting.")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("sed")
                        .short('s')
                        .long("sed")
                        .value_name("SED")
                        .help("Replaced text.")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("Output directory.")
                        .required(false)
                        .takes_value(true)
                        .default_value(current_dir_str),
                ),
        )
        .subcommand(
            Command::new("sum")
                .about("SUM CSV Column.")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("FILE")
                        .help("Target text file (csv, txt)")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("colname")
                        .short('c')
                        .long("column")
                        .value_name("COLUMN")
                        .help("CSV sum by column.")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            Command::new("ctoj")
                .about("CSV to JSON")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("FILE")
                        .help("Target CSV file (csv, txt)")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            Command::new("lastrow")
                .about("Get last row for text file.")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("FILE")
                        .help("Target CSV file (csv, txt)")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            Command::new("wc")
                .about("Word count.")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("FILE")
                        .help("Target text file.")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("chars")
                        .short('m')
                        .long("chars")
                        .value_name("CHARS COUNT")
                        .help("Text file chars count.")
                        .required(false)
                        .takes_value(false)
                )
                .arg(
                    Arg::new("lines")
                        .short('l')
                        .long("lines")
                        .value_name("LINE COUNT")
                        .help("Text file lines count.(Does not include line breaks)")
                        .required(false)
                        .takes_value(false)
                ),
        )
        .get_matches();


    // "split" ex) rustex split -t ./testfile/test1.txt -r "^第[1-9]章"
    if let Some(matches) = matches.subcommand_matches("split") {
        if let (Some(target_file), Some(regex_pattrern), Some(output_dir)) = (
            matches.value_of("target"),
            matches.value_of("regex"),
            matches.value_of("output"),
        ) {
            match split_file(target_file, regex_pattrern, output_dir) {
                Ok(message) => println!("{}", message),
                Err(e) => println!("{}", e),
            }
        }

    // "sortcsv" ex) rustex sortcsv -t ./testfile/test2.csv -c id
    // "sortcsv" ex) rustex sortcsv -t ./testfile/test2.csv -c id -r
    } else if let Some(matches) = matches.subcommand_matches("sortcsv") {
        if let (Some(target_file), Some(column_name), is_reverse) = (
            matches.value_of("target"),
            matches.value_of("colname"),
            matches.contains_id("reverse"),
        ) {
            if is_reverse {
                // 降順ソート
                match sort_csv_by_column(target_file, column_name, true) {
                    Ok(_) => {return},
                    Err(e) => println!("{}", e)
                }
            } else {
                // 昇順ソート
                match sort_csv_by_column(target_file, column_name, false) {
                    Ok(_) => {return},
                    Err(e) => println!("{}", e)
                }
            }
        }

    // "groupby" ex) rustex groupby -t ./testfile/test2.csv -c name
    } else if let Some(matches) = matches.subcommand_matches("groupby") {
        if let (Some(target_file), Some(column_name)) = (
            matches.value_of("target"),
            matches.value_of("colname"),
        ) {
            match groupby_column_csv(target_file, column_name) {
                Ok(_) => {return},
                Err(e) => println!("{}", e), 
            }
        }

    // "aggregate" ex) rustex aggregate -t ./testfile/test2.csv -k name -c score
    } else if let Some(matches) = matches.subcommand_matches("aggregate") {
        if let (Some(target_file), Some(key_column), Some(columns), floatmode, is_csv) = (
            matches.value_of("target"),
            matches.value_of("keycol"),
            matches.get_many::<String>("columns"),
            matches.contains_id("floatmode"),
            matches.contains_id("iscsv"),
         ) {
            let columns_str: Vec<&str> = columns.map(|c| c.as_str()).collect();
            if floatmode {
                if is_csv {
                    match aggregate_csv_data(target_file, key_column, &columns_str, true, true) {
                        Ok(_) => {return},
                        Err(e) => println!("{}", e)
                    }
                } else {
                    match aggregate_csv_data(target_file, key_column, &columns_str, true, false) {
                        Ok(_) => {return},
                        Err(e) => println!("{}", e)
                    }
                }
            } else {
                if is_csv {
                    match aggregate_csv_data(target_file, key_column, &columns_str, false, true) {
                        Ok(_) => {return},
                        Err(e) => println!("{}", e)
                    }
                } else {
                    match aggregate_csv_data(target_file, key_column, &columns_str, false, false) {
                        Ok(_) => {return},
                        Err(e) => println!("{}", e)
                    }
                }
            }
        }

    // "head" ex) rustex head -t ./testfile/test2.csv -l 10
    // "head" ex) rustex head -t ./testfile/test2.csv -c
    } else if let Some(matches) = matches.subcommand_matches("head") {
        if let (Some(target_file), iscsv) = (
            matches.value_of("target"),
            matches.contains_id("iscsv"),
        ) {
            if iscsv {
                match print_header_csv(target_file) {
                    Ok(_) => {return},
                    Err(e) => println!("{}", e)
                }
            } else {
                if let Some(read_limit) = matches.get_one::<usize>("limit").copied() {
                    match read_limit {
                        n => match print_head(target_file, n) {
                            Ok(_) => {return},
                            Err(e) => println!("{}", e)
                        }
                    };
                }
            }
        }

    // "excol" ex) rustex excol -t ./testfile/test2.csv -c name score
    } else if let Some(matches) = matches.subcommand_matches("excol") {
        if let (Some(target_file), Some(columns), Some(output_dir)) = (
            matches.value_of("target"),
            matches.get_many::<String>("columns"),
            matches.value_of("output"),
         ) {
            let columns_str: HashSet<&str> = columns.map(|c| c.as_str()).collect();
            match extract_column(target_file, columns_str, output_dir) {
                Ok(message) => println!("{}", message),
                Err(e) => println!("{}", e),
            }
        }
    
    // "clean" ex) rustex clean -t ./testfile/test2.csv -r "^[2-3],"
    } else if let Some(matches) = matches.subcommand_matches("clean") {
        if let (Some(target_file), Some(regex_pattrern), Some(output_dir)) = (
            matches.value_of("target"),
            matches.value_of("regex"),
            matches.value_of("output"),
        ) {
            match clean_row(target_file, regex_pattrern, output_dir) {
                Ok(message) => println!("{}", message),
                Err(e) => println!("{}", e),
            }
        }

    // "collect" ex) rustex collect -t ./test -r "maru"
    // "collect" ex) rustex collect -t ./test -r "^maru" ./collect
    } else if let Some(matches) = matches.subcommand_matches("collect") {
        if let (Some(target_dir), Some(output_dir), Some(regex_pattern)) = (
            matches.value_of("target"),
            matches.value_of("output"),
            matches.value_of("regex"),
        ) {
            match collect_file(target_dir, output_dir, regex_pattern) {
                Ok(message) => println!("{}", message),
                Err(e) => println!("{}", e)
            }
        }

    // "grep" ex) rustex grep -t ./testfile/test1.txt -r ^これは`
    // "grep" ex) rustex grep -t ./testfile/test2.csv -r ^1,` -c
    } else if let Some(matches) = matches.subcommand_matches("grep") {
        if let (Some(target_file), Some(regex_pattern), Some(output_dir), csv_header) = (
            matches.value_of("target"),
            matches.value_of("regex"),
            matches.value_of("output"),
            matches.contains_id("csv"),
        ) {
            if csv_header {
                match grep_row(target_file, regex_pattern, output_dir, true) {
                    Ok(message) => println!("{}", message),
                    Err(e) => println!("{}", e)
                }
            } else {
                match grep_row(target_file, regex_pattern, output_dir, false) {
                    Ok(message) => println!("{}", message),
                    Err(e) => println!("{}", e)
                }
            }
        }

    // "blocksplit" ex) rustex blocksplit -t ./testfile/test3-blocksplit.txt -c id
    } else if let Some(matches) = matches.subcommand_matches("blocksplit") {
        if let (Some(target_file), Some(target_column), Some(output_directory)) = (
            matches.value_of("target"),
            matches.value_of("colname"),
            matches.value_of("output"),
        ) {
            match block_split(target_file, target_column, output_directory) {
                Ok(message) => println!("{}", message),
                Err(e) => println!("{}", e)
            }
        }

    // "red" ex) rustex red -t ./testfile/test4-red.txt -r "Rust" -s "Rust言語"
    } else if let Some(matches) = matches.subcommand_matches("red") {
        if let (Some(target_file), Some(regex_pattern), Some(replaced_text), Some(output_directory)) = (
            matches.value_of("target"),
            matches.value_of("regex"),
            matches.value_of("sed"),
            matches.value_of("output"),
        ) {
            match red(target_file, regex_pattern, replaced_text, output_directory) {
                Ok(message) => println!("{}", message),
                Err(e) => println!("{}", e)
            }
        }
    
    // "sum" rustex sum -t ./testfile/test2.csv -c score
    } else if let Some(matches) = matches.subcommand_matches("sum") {
        if let (Some(target_file), Some(column_name)) = (
            matches.value_of("target"),
            matches.value_of("colname"),
        ) {
            match sum(target_file, column_name) {
                Ok(_) => {return},
                Err(e) => println!("{}", e)
            }
        }
    
    // "ctoj" ex) rustex ctoj -t ./testfile/test5-ctoj.csv
    } else if let Some(matches) = matches.subcommand_matches("ctoj") {
        if let Some(target_file) = matches.value_of("target") {
            match csv_to_json(target_file) {
                Ok(_) => {return},
                Err(e) => println!("{}", e)
            }
        }
    
    // "lastrow" ex) rustex lastrow -t ./testfile/test1.txt
    } else if let Some(matches) = matches.subcommand_matches("lastrow") {
        if let Some(target_file) = matches.value_of("target") {
            match get_last_row(target_file) {
                Ok(_) => {return},
                Err(e) => println!("{}", e)
            }
        }
    
    // "wc" ex) rustex lastrow -t ./testfile/test1.txt -l
    // "wc" ex) rustex lastrow -t ./testfile/test1.txt -m
    } else if let Some(matches) = matches.subcommand_matches("wc") {
        if let (Some(target_file), chars, lines) = (
            matches.value_of("target"),
            matches.contains_id("chars"),
            matches.contains_id("lines"),
        
        ) {
            if chars {
                match word_count(target_file) {
                    Ok(_) => {return},
                    Err(e) => println!("{}", e)
                }
            } else if lines {
                match line_count(target_file) {
                    Ok(_) => {return},
                    Err(e) => println!("{}", e)
                }
            }
        }
    }
}
