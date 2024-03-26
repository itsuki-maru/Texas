use csv::ReaderBuilder;
use std::collections::HashMap;
use std::fs::File;

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

    // 各キーに対する合計と件数を保持するHashMap
    let mut data: HashMap<&str, HashMap<String, (f64, u32)>> = HashMap::new();

    // レコードをイテレート
    for result in reader.records() {
        let record = result.expect("CSV Error.");
        let key = record.get(key_column_index).unwrap_or_default();
        for &column in target_columns {
            // key_columnのインデックスを見つける
            let key_column = headers.iter().position(|h| h == key_column)
                .ok_or_else(|| format!("Column name: `{}` not found.", key_column)).expect("Error");
            if let Some(value_str) = record.get(key_column) {
                let value = if floatmode {
                    value_str.parse::<f64>().unwrap_or(0.0)
                } else {
                    value_str.parse::<f64>().unwrap_or(0.0) as f64
                };
                let entry = data.get_mut(column).unwrap().entry(key.to_string()).or_insert((0.0, 0));
                entry.0 += value;
                entry.1 += 1;
            }
        }
    }

    // 結果を出力
    for &column in target_columns {
        println!("Column: {}", column);
        for (key, (sum, count)) in data.get(column).unwrap().iter() {
            println!("Key: {}, Sum: {}, Count: {}", key, sum, count);
        }
    }

    StatusData {
        status_code: 200,
        message: "File read successfully.".to_string(),
    }
}