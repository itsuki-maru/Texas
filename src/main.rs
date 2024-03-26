use clap::{App, Arg, Command};

mod rustex;
mod scheme;
mod utils;
use rustex::aggregate::aggregate_csv_data;
use rustex::sortcsv::sort_csv_by_column;
use rustex::split::split_file;
use rustex::groupby::groupby_column_csv;
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
                        .value_name("FLOATM ODE")
                        .help("Aggregate float.")
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
            let result = split_file(target_file, regex_pattrern, output_dir);
            println!(
                "Status Code: {}, Message: {}",
                result.status_code, result.message
            );
            return;
        }
    } else if let Some(matches) = matches.subcommand_matches("sortcsv") {
        if let (Some(target_file), Some(column_name), is_reverse) = (
            matches.value_of("target"),
            matches.value_of("colname"),
            matches.contains_id("reverse"),
        ) {
            if is_reverse {
                // 降順ソート
                let _ = sort_csv_by_column(target_file, column_name, true);   
            } else {
                // 昇順ソート
                let _ = sort_csv_by_column(target_file, column_name, false);   
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("groupby") {
        if let (Some(target_file), Some(column_name)) = (
            matches.value_of("target"),
            matches.value_of("colname"),
        ) {
            let _ = groupby_column_csv(target_file, column_name);
        }
    } else if let Some(matches) = matches.subcommand_matches("aggregate") {
        if let (Some(target_file), Some(key_column), Some(columns), floatmode) = (
            matches.value_of("target"),
            matches.value_of("keycol"),
            matches.get_many::<String>("columns"),
            matches.contains_id("floatmode"),
         ) {
            let columns_str: Vec<&str> = columns.map(|c| c.as_str()).collect();
            if floatmode {
                let _ = aggregate_csv_data(target_file, key_column, &columns_str, true);
            } else {
                let _ = aggregate_csv_data(target_file, key_column, &columns_str, false);
            }
        }
    }
}
