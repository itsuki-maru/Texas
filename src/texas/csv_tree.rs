use csv::ReaderBuilder;
use std::collections::HashMap;
use serde::Serialize;
use anyhow::{Result, anyhow};
use super::super::utils::{get_abs_filepath, is_file};

#[derive(Debug, Serialize)]
struct Entry {
    records: Vec<HashMap<String, String>>,
    counts: HashMap<String, HashMap<String, usize>>,
    sum: HashMap<String, f64>,
}

// CSVファイルを集計
pub fn csv_tree(
    target_file: &str,
    category_column: &str,
    key_column: &str,
    count_columns: Vec<&str>,
    sum_columns: Vec<&str>,
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

    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .from_path(target_file_abs)?;

    let headers = reader.headers()?.clone();
    let mut data_dict: HashMap<String, HashMap<String, Entry>> = HashMap::new();

    for result in reader.records() {
        let record = result?;
        let row: HashMap<String, String> = headers
            .iter()
            .zip(record.iter())
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        let category = row.get(category_column).cloned().unwrap_or_default();
        let name = row.get(key_column).cloned().unwrap_or_default();

        // Entryの取得と初期化
        let name_map = data_dict.entry(category).or_default();
        let entry = name_map.entry(name).or_insert_with(|| Entry {
            records: vec![],
            counts: HashMap::new(),
            sum: HashMap::new(),
        });

        // 元の行を記録
        entry.records.push(row.clone());

        // counts（文字列出現回数）
        for col in &count_columns {
            let value = row.get(*col).cloned().unwrap_or_default();
            let col_map = entry.counts.entry(col.to_string()).or_default();
            *col_map.entry(value).or_insert(0) += 1;
        }

        // sum（数値合計）
        for col in &sum_columns {
            if let Some(val) = row.get(*col) {
                if let Ok(num) = val.parse::<f64>() {
                    *entry.sum.entry(col.to_string()).or_insert(0.0) += num;
                }
            }
        }

    }

    // JSONとして書き出し
    let json_output = serde_json::to_string_pretty(&data_dict)?;
    println!("{}", json_output);

    Ok(())
}
