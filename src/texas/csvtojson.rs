use csv::ReaderBuilder;
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use anyhow::{Result, anyhow};
use super::super::utils::{
    get_abs_filepath,
    is_file,
};

// データを格納するための構造体
#[derive(Serialize)]
struct Record {
    data: Vec<HashMap<String, String>>,
}

pub fn csv_to_json(
    target_file: &str,
) ->Result<()> {

    // 対象ファイルの絶対パスを取得
    let target_file_abs = match get_abs_filepath(target_file) {
        Ok(path) => path,
        Err(_) => return Err(anyhow!("File absolute path get error."))
    };

    // ファイルの存在確認
    if !target_file_abs.exists() {
        return Err(anyhow!(format!("{} is not exists.", target_file)))
    }

    // 指定されたファイルがファイルであることを確認
    if !is_file(&target_file_abs) {
        return Err(anyhow!(format!("{} is not text file.", target_file)))
    }

    // CSVファイルを読み込み
    let file = match File::open(target_file_abs) {
        Ok(file) => file,
        Err(_) => return Err(anyhow!("CSV file read error."))
    };
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    // ヘッダーを取得
    let headers = reader.headers()?
        .clone();

    // データを格納するベクタを初期化
    let mut records = Vec::new();

    for result in reader.records() {
        let record = result?;
        let mut data_record: HashMap<String, serde_json::Value> = HashMap::new();

        for (idx, header) in headers.iter().enumerate() {
            if let Some(value) = record.get(idx) {
                if !value.is_empty() {
                    // 同名のヘッダーが存在する場合、値を配列に格納する
                    match data_record.get_mut(header) {
                        Some(serde_json::Value::Array(arr)) => {
                            arr.push(serde_json::Value::String(value.to_string()));
                        },
                        Some(existing_value) => {
                            // 既存の値が配列でない場合、配列に変換する
                            let new_array = vec![existing_value.take(), serde_json::Value::String(value.to_string())];
                            data_record.insert(header.to_string(), serde_json::Value::Array(new_array));
                        },
                        None => {
                            data_record.insert(header.to_string(), serde_json::Value::String(value.to_string()));
                        }
                    }
                }
            }
        }
        records.push(data_record);
    }

    // レコードをJSON文字列に変換
    let serialized_json = serde_json::to_string(&records)?;

    println!("{}", serialized_json);

    Ok(())
}