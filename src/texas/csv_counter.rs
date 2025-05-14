use csv::ReaderBuilder;
use std::collections::HashMap;
use anyhow::{Result, anyhow};
use super::super::utils::{get_abs_filepath, is_file};


// CSVファイルを集計
pub fn csv_counter(
    target_file: &str,
    key_column: &str,
    name_column: &str,
    target_columns: Vec<&str>,
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

    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .from_path(target_file_abs)?;

    let headers = rdr.headers()?.clone();
    let mut data_dict: HashMap<String, HashMap<String, HashMap<String, HashMap<String, usize>>>> = HashMap::new();

    for result in rdr.records() {
        let record = result?;
        let row: HashMap<String, String> = headers
            .iter()
            .zip(record.iter())
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        let category = row.get(key_column).cloned().unwrap_or_default();
        let name = row.get(name_column).cloned().unwrap_or_default();

        let name_map = data_dict.entry(category).or_default();
        let start_map = name_map.entry(name).or_default();

        for col in &target_columns {
            let value = row.get(*col).cloned().unwrap_or_default();
            let counter = start_map.entry(col.to_string()).or_default();
            *counter.entry(value).or_insert(0) += 1;
        }

    }

    // JSONとして書き出し
    let json_output = serde_json::to_string_pretty(&data_dict)?;
    println!("{}", json_output);

    Ok(())
}
