use csv::ReaderBuilder;
use std::collections::HashMap;
use std::fs::File;
use std::error::Error;

use crate::scheme::StatusData;
use super::super::utils::{
    get_abs_filepath,
    is_file
};

pub fn agrregate_csv_data(
    target_file: &str,
    key_column: &str,
    target_columns: &[&str],
    floatmode: bool,
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
            return  StatusData {
                status_code: 200,
                message: "File read error.".to_string(),
            };
        }
    };
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    // ヘッダーを取得
    let headers = reader.headers().expect("Header get error.").clone();

    // key_columnのインデックスを見つける
    let key_column_index = headers.iter().position(|h| h == key_column)
        .ok_or_else(|| format!("Column name: `{}` not found.", key_column)).expect("Error");

    for result in reader.records() {
        let record = result.expect("CSV Error.");
        let key = record.get(key_column_index).unwrap_or_default();
    }

    StatusData {
        status_code: 200,
        message: "File read successfully.".to_string(),
    }
}