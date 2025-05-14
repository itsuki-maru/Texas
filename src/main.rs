use clap::{Arg, Command};
use std::collections::HashSet;
mod texas;
mod utils;
use texas::{
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
    csv_tree::csv_tree,
};

fn main() {
    let matches = Command::new("texas")
        .version("1.1.0")
        .author("Itsuki Maru")
        .about("Text file proessing tool.")
        .subcommand(
            Command::new("texas")
                .about("Welcome!!")
        )
        .subcommand(
            Command::new("split")
                .about("Splits a file based on a reguler expression.")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("FILE")
                        .help("ex) -t ./testfile/test1.txt")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("regex")
                        .short('r')
                        .long("regex")
                        .value_name("REGEX")
                        .help("ex) -r ^第[1-9]章")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("ex) -o ~/Desktop")
                        .required(false)
                        .value_parser(clap::value_parser!(String))
                        .default_value("./"),
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
                        .help("ex) -t ./testfile/test2.csv")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("colname")
                        .short('c')
                        .long("column")
                        .value_name("COLUMN")
                        .help("ex) -c id")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("reverse")
                        .short('r')
                        .long("reverse")
                        .value_name("REVERSE")
                        .help("ex) -r")
                        .required(false)
                        .action(clap::ArgAction::SetTrue)
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
                        .help("ex) -t ./testfile/test2.csv")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("colname")
                        .short('c')
                        .long("column")
                        .value_name("COLUMN")
                        .help("ex) -c name")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
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
                        .help("ex) -t ./testfile/test2.csv")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("keycol")
                        .short('k')
                        .long("keycolumn")
                        .value_name("COLUMN")
                        .help("ex) -k name")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("columns")
                        .short('c')
                        .long("columns")
                        .value_name("TARGET COLUMNS")
                        .help("ex) -c score")
                        .required(true)
                        .value_parser(clap::value_parser!(String))
                        .num_args(1..)
                )
                .arg(
                    Arg::new("floatmode")
                        .short('f')
                        .long("floatmode")
                        .value_name("FLOAT MODE")
                        .help("ex) -f")
                        .required(false)
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("iscsv")
                        .short('i')
                        .long("iscsv")
                        .value_name("OUTPUT CSV")
                        .help("ex) -i")
                        .required(false)
                        .action(clap::ArgAction::SetTrue)
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
                        .help("ex) -t ./testfile/test2.csv")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("iscsv")
                        .short('c')
                        .long("iscsv")
                        .value_name("CSV CODE")
                        .help("ex) -c")
                        .required(false)
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("limit")
                        .short('l')
                        .long("limit")
                        .value_name("FILE")
                        .help("ex) -l 10")
                        .required(false)
                        .value_parser(clap::value_parser!(String))
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
                        .help("ex) -t ./testfile/test2.csv")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("columns")
                        .short('c')
                        .long("columns")
                        .value_name("TARGET COLUMNS")
                        .help("ex) -c name score")
                        .required(true)
                        .value_parser(clap::value_parser!(String))
                        .num_args(1..)
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("ex) ~/Desktop")
                        .required(false)
                        .value_parser(clap::value_parser!(String))
                        .default_value("./"),
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
                        .help("ex) -t ./testfile/test2.csv")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("regex")
                        .short('r')
                        .long("regex")
                        .value_name("REGEX")
                        .help("ex) -r ^[2-3],")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("ex) ~/Desktop")
                        .required(false)
                        .value_parser(clap::value_parser!(String))
                        .default_value("./"),
                ),
        )
        .subcommand(
            Command::new("collect")
                .about("Collect file on a reguler expression.")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("Directory")
                        .help("ex) -t ./testfile/collect_test")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("ex) -o ~/Desktop")
                        .required(false)
                        .value_parser(clap::value_parser!(String))
                        .default_value("./"),
                )
                .arg(
                    Arg::new("regex")
                        .short('r')
                        .long("regex")
                        .value_name("REGEX")
                        .help("ex) -r ^maru")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
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
                        .help("ex) -t ./testfile/test1.txt")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("regex")
                        .short('r')
                        .long("regex")
                        .value_name("REGEX")
                        .help("ex) -r ^これは")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("ex) -o ~/Desktop")
                        .required(false)
                        .value_parser(clap::value_parser!(String))
                        .default_value("./"),
                )
                .arg(
                    Arg::new("csv")
                        .short('c')
                        .long("csv")
                        .value_name("CSV HEADER")
                        .help("ex) -c")
                        .required(false)
                        .action(clap::ArgAction::SetTrue)
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
                        .help("ex) -t ./testfile/test3-blocksplit.txt")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("colname")
                        .short('c')
                        .long("column")
                        .value_name("COLUMN")
                        .help("ex) -c id")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("ex) -o ~/Desktop")
                        .required(false)
                        .value_parser(clap::value_parser!(String))
                        .default_value("./"),
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
                        .help("ex) -t ./testfile/test4-red.txt")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("regex")
                        .short('r')
                        .long("regex")
                        .value_name("REGEX")
                        .help("ex) -r Rust")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("sed")
                        .short('s')
                        .long("sed")
                        .value_name("SED")
                        .help("ex) -s Rust言語")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("ex) -o ~/Desktop")
                        .required(false)
                        .value_parser(clap::value_parser!(String))
                        .default_value("./"),
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
                        .help("ex) -t ./testfile/test2.csv")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("colname")
                        .short('c')
                        .long("column")
                        .value_name("COLUMN")
                        .help("ex) -c score")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
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
                        .help("ex) -t ./testfile/test5-ctoj.csv")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
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
                        .help("ex) -t ./testfile/test1.txt")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
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
                        .help("ex) -t ./testfile/test1.txt")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("chars")
                        .short('m')
                        .long("chars")
                        .value_name("CHARS COUNT")
                        .help("ex) -m")
                        .required(false)
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("lines")
                        .short('l')
                        .long("lines")
                        .value_name("LINE COUNT")
                        .help("ex) -l")
                        .required(false)
                        .action(clap::ArgAction::SetTrue)
                ),
        )
        .subcommand(
            Command::new("csvtree")
                .about("CSV to tree mapping.")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("FILE")
                        .help("ex) -t ./testfile/test6.csv")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("category")
                        .short('c')
                        .long("category")
                        .value_name("COLUMN")
                        .help("ex) -c category")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("key")
                        .short('k')
                        .long("key")
                        .value_name("KEY COLUMN")
                        .help("ex) -k name")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("count")
                        .short('C')
                        .long("count")
                        .value_name("COUNT TARGET COLUMNS")
                        .help("ex) -C origin grade size")
                        .required(true)
                        .value_parser(clap::value_parser!(String))
                        .num_args(1..)
                )
                .arg(
                    Arg::new("sum")
                        .short('S')
                        .long("SUM")
                        .value_name("SUM TARGET COLUMNS")
                        .help("ex) -S size")
                        .required(true)
                        .value_parser(clap::value_parser!(String))
                        .num_args(1..)
                )
        )
        .get_matches();

    // Welcome Command
    if let Some(_) = matches.subcommand_matches("texas") {
        let texs = vec![
            "\n+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+",
            "+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+",
            "-----/\\----\\------------------/\\----\\-----------------______----------------/\\----\\------------------/\\----\\---------",
            "----/::\\----\\----------------/::\\----\\---------------|::|---|--------------/::\\----\\----------------/::\\----\\--------",
            "----\\:::\\----\\--------------/::::\\----\\--------------|::|---|-------------/::::\\----\\--------------/::::\\----\\-------",
            "-----\\:::\\----\\------------/::::::\\----\\-------------|::|---|------------/::::::\\----\\------------/::::::\\----\\------",
            "------\\:::\\----\\----------/:::/\\:::\\----\\------------|::|---|-----------/:::/\\:::\\----\\----------/:::/\\:::\\----\\-----",
            "-------\\:::\\----\\--------/:::/__\\:::\\----\\-----------|::|---|----------/:::/__\\:::\\----\\--------/:::/__\\:::\\----\\----",
            "-------/::::\\----\\------/::::\\---\\:::\\----\\----------|::|---|---------/::::\\---\\:::\\----\\-------\\:::\\---\\:::\\----\\---",
            "------/::::::\\----\\----/::::::\\---\\:::\\----\\---------|::|---|--------/::::::\\---\\:::\\----\\----___\\:::\\---\\:::\\----\\--",
            "-----/:::/\\:::\\----\\--/:::/\\:::\\---\\:::\\----\\--______|::|___|___-__-/:::/\\:::\\---\\:::\\----\\--/\\---\\:::\\---\\:::\\----\\-",
            "----/:::/--\\:::\\____\\/:::/__\\:::\\---\\:::\\____\\|:::::::::::::::::|--|:::/--\\:::\\---\\:::\\____\\/::\\---\\:::\\---\\:::\\____\\",
            "---/:::/----\\::/----/\\:::\\---\\:::\\---\\::/----/|:::::::::::::::::|__|::/----\\:::\\--/:::/----/\\:::\\---\\:::\\---\\::/----/",
            "--/:::/----/-\\/____/--\\:::\\---\\:::\\---\\/____/--------|::|---|-------\\/____/-\\:::\\/:::/----/--\\:::\\---\\:::\\---\\/____/-",
            "-/:::/----/------------\\:::\\---\\:::\\----\\------------|::|---|----------------\\::::::/----/----\\:::\\---\\:::\\----\\-----",
            "/:::/----/--------------\\:::\\---\\:::\\____\\-----------|::|---|-----------------\\::::/----/------\\:::\\---\\:::\\____\\----",
            "\\::/----/----------------\\:::\\---\\::/----/-----------|::|---|-----------------/:::/----/--------\\:::\\--/:::/----/----",
            "-\\/____/------------------\\:::\\---\\/____/------------|::|---|----------------/:::/----/----------\\:::\\/:::/----/-----",
            "---------------------------\\:::\\----\\----------------|::|---|---------------/:::/----/------------\\::::::/----/------",
            "----------------------------\\:::\\____\\---------------|::|---|--------------/:::/----/--------------\\::::/----/-------",
            "-----------------------------\\::/----/---------------|::|___|--------------\\::/----/----------------\\::/----/--------",
            "------------------------------\\/____/---------------------------------------\\/____/------------------\\/____/---------",
            "+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+",
            "+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+\n",
        ];
        for row in texs.iter() {
            println!("{}", row);
        }

     // "split" ex) texas split -t ./testfile/test1.txt -r "^第[1-9]章"
    } else if let Some(matches) = matches.subcommand_matches("split") {
        if let (Some(target_file), Some(regex_pattrern), Some(output_dir)) = (
            matches.get_one::<String>("target"),
            matches.get_one::<String>("regex"),
            matches.get_one::<String>("output"),
        ) {
            match split_file(target_file, regex_pattrern, output_dir) {
                Ok(message) => println!("{}", message),
                Err(e) => println!("{}", e),
            }
        }

    // "sortcsv" ex) texas sortcsv -t ./testfile/test2.csv -c id
    // "sortcsv" ex) texas sortcsv -t ./testfile/test2.csv -c id -r
    } else if let Some(matches) = matches.subcommand_matches("sortcsv") {
        if let (Some(target_file), Some(column_name), is_reverse) = (
            matches.get_one::<String>("target"),
            matches.get_one::<String>("colname"),
            matches.get_flag("reverse"),
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

    // "groupby" ex) texas groupby -t ./testfile/test2.csv -c name
    } else if let Some(matches) = matches.subcommand_matches("groupby") {
        if let (Some(target_file), Some(column_name)) = (
            matches.get_one::<String>("target"),
            matches.get_one::<String>("colname"),
        ) {
            match groupby_column_csv(target_file, column_name) {
                Ok(_) => {return},
                Err(e) => println!("{}", e), 
            }
        }

    // "aggregate" ex) texas aggregate -t ./testfile/test2.csv -k name -c score
    } else if let Some(matches) = matches.subcommand_matches("aggregate") {
        if let (Some(target_file), Some(key_column), Some(columns), floatmode, is_csv) = (
            matches.get_one::<String>("target"),
            matches.get_one::<String>("keycol"),
            matches.get_many::<String>("columns"),
            matches.get_flag("floatmode"),
            matches.get_flag("iscsv"),
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

    // "head" ex) texas head -t ./testfile/test2.csv -l 10
    // "head" ex) texas head -t ./testfile/test2.csv -c
    } else if let Some(matches) = matches.subcommand_matches("head") {
        if let (Some(target_file), iscsv) = (
            matches.get_one::<String>("target"),
            matches.get_flag("iscsv"),
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

    // "excol" ex) texas excol -t ./testfile/test2.csv -c name score
    } else if let Some(matches) = matches.subcommand_matches("excol") {
        if let (Some(target_file), Some(columns), Some(output_dir)) = (
            matches.get_one::<String>("target"),
            matches.get_many::<String>("columns"),
            matches.get_one::<String>("output"),
        ) {
            let columns_str: HashSet<&str> = columns.map(|c| c.as_str()).collect();
            match extract_column(target_file, columns_str, output_dir) {
                Ok(message) => println!("{}", message),
                Err(e) => println!("{}", e),
            }
        }
    
    // "clean" ex) texas clean -t ./testfile/test2.csv -r "^[2-3],"
    } else if let Some(matches) = matches.subcommand_matches("clean") {
        if let (Some(target_file), Some(regex_pattrern), Some(output_dir)) = (
            matches.get_one::<String>("target"),
            matches.get_one::<String>("regex"),
            matches.get_one::<String>("output"),
        ) {
            match clean_row(target_file, regex_pattrern, output_dir) {
                Ok(message) => println!("{}", message),
                Err(e) => println!("{}", e),
            }
        }

    // "collect" ex) texas collect -t ./testfile/collect_test -r "maru"
    // "collect" ex) texas collect -t ./testfile/collect_test -r "^maru" -o ~/Desktop
    } else if let Some(matches) = matches.subcommand_matches("collect") {
        if let (Some(target_dir), Some(output_dir), Some(regex_pattern)) = (
            matches.get_one::<String>("target"),
            matches.get_one::<String>("output"),
            matches.get_one::<String>("regex"),
        ) {
            match collect_file(target_dir, output_dir, regex_pattern) {
                Ok(message) => println!("{}", message),
                Err(e) => println!("{}", e)
            }
        }

    // "grep" ex) texas grep -t ./testfile/test1.txt -r ^これは
    // "grep" ex) texas grep -t ./testfile/test2.csv -r ^1, -c
    } else if let Some(matches) = matches.subcommand_matches("grep") {
        if let (Some(target_file), Some(regex_pattern), Some(output_dir), csv_header) = (
            matches.get_one::<String>("target"),
            matches.get_one::<String>("regex"),
            matches.get_one::<String>("output"),
            matches.get_flag("csv"),
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

    // "blocksplit" ex) texas blocksplit -t ./testfile/test3-blocksplit.txt -c id
    } else if let Some(matches) = matches.subcommand_matches("blocksplit") {
        if let (Some(target_file), Some(target_column), Some(output_directory)) = (
            matches.get_one::<String>("target"),
            matches.get_one::<String>("colname"),
            matches.get_one::<String>("output"),
        ) {
            match block_split(target_file, target_column, output_directory) {
                Ok(message) => println!("{}", message),
                Err(e) => println!("{}", e)
            }
        }

    // "red" ex) texas red -t ./testfile/test4-red.txt -r "Rust" -s "Rust言語"
    } else if let Some(matches) = matches.subcommand_matches("red") {
        if let (Some(target_file), Some(regex_pattern), Some(replaced_text), Some(output_directory)) = (
            matches.get_one::<String>("target"),
            matches.get_one::<String>("regex"),
            matches.get_one::<String>("sed"),
            matches.get_one::<String>("output"),
        ) {
            match red(target_file, regex_pattern, replaced_text, output_directory) {
                Ok(message) => println!("{}", message),
                Err(e) => println!("{}", e)
            }
        }
    
    // "sum" texas sum -t ./testfile/test2.csv -c score
    } else if let Some(matches) = matches.subcommand_matches("sum") {
        if let (Some(target_file), Some(column_name)) = (
            matches.get_one::<String>("target"),
            matches.get_one::<String>("colname"),
        ) {
            match sum(target_file, column_name) {
                Ok(_) => {return},
                Err(e) => println!("{}", e)
            }
        }
    
    // "ctoj" ex) texas ctoj -t ./testfile/test5-ctoj.csv
    } else if let Some(matches) = matches.subcommand_matches("ctoj") {
        if let Some(target_file) = matches.get_one::<String>("target") {
            match csv_to_json(target_file) {
                Ok(_) => {return},
                Err(e) => println!("{}", e)
            }
        }
    
    // "lastrow" ex) texas lastrow -t ./testfile/test1.txt
    } else if let Some(matches) = matches.subcommand_matches("lastrow") {
        if let Some(target_file) = matches.get_one::<String>("target") {
            match get_last_row(target_file) {
                Ok(_) => {return},
                Err(e) => println!("{}", e)
            }
        }
    
    // "wc" ex) texas wc -t ./testfile/test1.txt -l
    // "wc" ex) texas wc -t ./testfile/test1.txt -m
    } else if let Some(matches) = matches.subcommand_matches("wc") {
        if let (Some(target_file), chars, lines) = (
            matches.get_one::<String>("target"),
            matches.get_flag("chars"),
            matches.get_flag("lines"),
        
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
    // "csvtree" ex) texas csvtree -t ./testfile/test6.csv -c category -k name -C origin grade size
    } else if let Some(matches) = matches.subcommand_matches("csvtree") {
        if let (Some(target_file), Some(key_column), Some(name_column), Some(count_columns), Some(sum_columns)) = (
            matches.get_one::<String>("target"),
            matches.get_one::<String>("category"),
            matches.get_one::<String>("key"),
            matches.get_many::<String>("count"),
            matches.get_many::<String>("sum"),
        ) {
            let count_columns_str: Vec<&str> = count_columns.map(|c| c.as_str()).collect();
            let sum_columns_str: Vec<&str> = sum_columns.map(|c| c.as_str()).collect();
            match csv_tree(target_file, key_column, name_column,count_columns_str, sum_columns_str) {
                Ok(_) => {return},
                Err(e) => println!("{}", e)
            }
        }
    }
}
