use csv::ReaderBuilder;
use std::fs::{self, File};
use crate::scheme::StatusData;
use super::super::utils::{
    get_abs_filepath,
    is_file,
};

// テキストファイルの先頭行を出力
pub fn print_head(
    target_file: &str,
    read_limit: usize,
) -> StatusData {

    // 対象ファイルの絶対パスを取得
    let target_file_abs = match get_abs_filepath(target_file) {
        Ok(path) => path,
        Err(_) => {
            return StatusData {
                status_code: 404,
                message: format!("{} is not found.", target_file),
            };
        }
    };

    // ファイルの存在確認
    if !target_file_abs.exists() {
        return StatusData {
            status_code: 404,
            message: format!("{} is not found.", target_file),
        };
    }

    // 指定されたファイルがファイルであることを確認
    if !is_file(&target_file_abs) {
        return StatusData {
            status_code: 400,
            message: format!("{} is not csv file.", target_file),
        };
    }

    // テキストファイルを読み込み
    let text = match fs::read_to_string(&target_file_abs) {
        Ok(content) => content,
        Err(_) => return StatusData {
            status_code: 500,
            message: "Failed to read the file.".to_string(),
        },
    };

    // 行を格納するベクタ
    let lines = text.split("\n").collect::<Vec<_>>();
    // 行番号
    let mut row_number = 1;
    for (i, line) in lines.iter().enumerate() {
        println!("{}:\t{}", row_number, line);
        row_number += 1;
        if i == read_limit-1 {
            break;
        }
    }

    StatusData {
        status_code: 200,
        message: "File read successfully.".to_string(),
    }
}

// CSVファイルのヘッダー行を出力
pub fn print_header_csv(
    target_file: &str,
) -> StatusData {
    // 対象ファイルの絶対パスを取得
    let target_file_abs = match get_abs_filepath(target_file) {
        Ok(path) => path,
        Err(_) => {
            return StatusData {
                status_code: 404,
                message: format!("{} is not found.", target_file),
            };
        }
    };

    // ファイルの存在確認
    if !target_file_abs.exists() {
        return StatusData {
            status_code: 404,
            message: format!("{} is not found.", target_file),
        };
    }

    // 指定されたファイルがファイルであることを確認
    if !is_file(&target_file_abs) {
        return StatusData {
            status_code: 400,
            message: format!("{} is not csv file.", target_file),
        };
    }

    // CSVファイルを読み込み
    let file = match File::open(target_file_abs) {
        Ok(file) => file,
        Err(_) => {
            return StatusData {
                status_code: 400,
                message: "File read error.".to_string(),
            };
        }
    };
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);
    // ヘッダーを取得
    let headers = reader.headers().expect("Header get error.").clone();

    for (i, header) in headers.iter().enumerate() {
        println!("{}:\t{}", i ,header);
    }


    StatusData {
        status_code: 200,
        message: "File read successfully.".to_string(),
    }
}