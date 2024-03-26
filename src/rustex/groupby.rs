use csv::ReaderBuilder;
use std::collections::HashMap;
use std::fs::File;

use crate::scheme::StatusData;
use super::super::utils::{
    get_abs_filepath,
    is_file
};


pub fn groupby_column_csv(
    target_file: &str,
    column_name: &str
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
    let headers = reader.headers().expect("Error.").clone();
    
    // 列名から列のインデックスを見つける
    let column_index = headers.iter().position(|h| h == column_name)
        .ok_or_else(|| format!("Column name: `{}` not found.", column_name)).expect("Error");

    // 出現回数をカウントするHashMapを初期化
    let mut counts: HashMap<String, u32> = HashMap::new();

    // レコードをイテレートし、指定列の値の出現回数をカウント
    for result in reader.records() {
        let record = result.expect("CSV Error.");
        let value = record.get(column_index).unwrap_or("").to_string();
        *counts.entry(value).or_insert(0) += 1;
    }

    // 出現回数を表示
    println!("===== TARGET COLUMN: {} =====", column_name);
    for (value, count) in counts {
        println!("{}:\t{}", value, count);
    }
    StatusData {
        status_code: 200,
        message: "Group count successfully.".to_string(),
    }
}