use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use std::fs::File;
use std::io;

use crate::scheme::StatusData;
use super::super::utils::{
    get_abs_filepath,
    is_file
};


pub fn sort_csv_by_column(
    target_file: &str,
    column_name: &str,
    is_reverse: bool,
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
        .ok_or_else(|| "指定された列名が見つかりません").expect("Error");

    // レコードを読み込む
    let mut records: Vec<StringRecord> = reader.records()
        .filter_map(Result::ok)
        .collect();

    // 指定された列でレコードをソート
    if is_reverse {
        records.sort_by(|a, b| b[column_index].cmp(&a[column_index]));
    } else {
        records.sort_by(|a, b| a[column_index].cmp(&b[column_index]));
    }

    // ソートされたレコードを標準出力
    let mut writer = WriterBuilder::new().from_writer(io::stdout());
    let _ = writer.write_record(&headers);

    for record in records {
        writer.write_record(&record).expect("Error.");
    }

    StatusData {
        status_code: 200,
        message: "CSV Sorted.".to_string()
    }
}