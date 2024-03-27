use std::fs::File;
use std::io::{self, BufRead, Write};
use regex::Regex;

use crate::scheme::StatusData;
use super::super::utils::{
    get_abs_filepath,
    get_abs_directory_path,
    is_dir,
};

// 正規表現に一致する行を削除
pub fn clean_row(
    target_file: &str,
    regex_pattern: &str,
    output_directory: &str,
) -> StatusData {

    // 対象ファイルの絶対パスを取得
    let target_file_abs = match get_abs_filepath(target_file) {
        Ok(path) => path,
        Err(_) => return StatusData {
            status_code: 404,
            message: format!("{} is not found.", target_file),
        },
    };

    // ファイルの存在確認
    if !target_file_abs.exists() {
        return StatusData {
            status_code: 404,
            message: format!("{} is not found.", target_file),    
        };
    }

    // 出力先の絶対パスを取得
    let output_directory_abs = match get_abs_directory_path(output_directory) {
        Ok(path) => path,
        Err(_) => return StatusData {
            status_code: 404,
            message: format!("{} is not found.", output_directory),
        },
    };

    // 出力先がディレクトリか確認
    if !is_dir(&output_directory_abs) {
        return StatusData {
            status_code: 400,
            message: format!("{} is not directory.", output_directory),    
        };
    }

    let file = File::open(target_file_abs).expect("File open error.");
    let reader = io::BufReader::new(file);
    let regex = Regex::new(regex_pattern).expect("Regex text error.");

    // 出力ファイル名
    let output_path = output_directory_abs.join("clean_row.txt");

    let mut output = File::create(output_path).expect("File create error.");

    for line in reader.lines() {
        let line = line.expect("Row get error.");
        if !regex.is_match(&line) {
            writeln!(output, "{}", line).expect("File write error.");
        }
    }

    StatusData {
        status_code: 200,
        message: "File read successfully.".to_string(),
    }
}