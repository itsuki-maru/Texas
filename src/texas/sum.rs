use csv::ReaderBuilder;
use std::fs::File;
use serde::Deserialize;
use anyhow::{Result, anyhow};
use super::super::utils::{get_abs_filepath, is_file, format_with_connma};

// 動的に型を扱うための列挙型
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Value {
    Float(f64),
    Int(i64),
    NA,
}

pub fn sum(
    target_file: &str,
    target_column: &str,
) -> Result<()> {

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

    // target_columnのインデックスを見つける
    let target_column_index = headers
        .iter()
        .position(|h| h == target_column)
        .ok_or_else(|| anyhow!("Column name: `{}` not found.", target_column))?;

    // sum: 最終的な集計値
    // count: データ件数（ヘッダー行を除外するためにここで加算）
    let mut sum: f64 = 0.0;
    let mut count: usize = 0;
    for result in reader.records() {
        let record = result.map_err(|e| anyhow!("CSV record get error.: {}", e))?;
        let value_str = &record[target_column_index];

        // Value型への変換を試みる（ヘッダー行の処理のため）
        let value = if let Ok(int_val) = value_str.parse::<i64>() {
            Value::Int(int_val)
        } else if let Ok(float_val) = value_str.parse::<f64>() {
            Value::Float(float_val)
        } else {
            // ヘッダー行であればNAとしてカウント
            Value::NA
        };

        match value {
            Value::Int(i) => sum += i as f64,
            Value::Float(f) => sum += f,
            Value::NA => sum += 0.0,
        }
        count += 1;
    }

    // total(f64)のフォーマット
    let total_integet_part = sum.trunc() as i64;
    let total_decimal_part = sum.fract();
    let formatted_total_integer = format_with_connma(total_integet_part);
    let formatted_total = format!("{}{}", formatted_total_integer, format!("{:.2}", total_decimal_part).trim_start_matches('0'));

    // count(usize)のフォーマット
    let formatted_count = format_with_connma(count as i64);

    println!("===== COLUMN: {} =====", target_column);
    println!("SUM: {} COUNT: {}", formatted_total, formatted_count);

    Ok(())
}