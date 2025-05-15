use csv::{ReaderBuilder, WriterBuilder};
use std::collections::HashSet;
use std::fs::File;
use std::io;
use anyhow::{Result, anyhow};
use super::super::utils::get_abs_filepath;
use super::super::scheme::SumMode;

// CSVファイルの指定した列名を加算して新しい列を追加
pub fn sum_column(
    target_file: &str,
    col_names: HashSet<&str>,
    new_column_name: &str,
    mode: SumMode,
) -> Result<String> {

    // 対象ファイルの絶対パスを取得
    let target_file_abs = match get_abs_filepath(target_file) {
        Ok(path) => path,
        Err(_) => return Err(anyhow!("File absolute path get error."))
    };

    // ファイルの存在確認
    if !target_file_abs.exists() {
        return Err(anyhow!(format!("{} is not exists.", target_file)))
    }

    // CSVリーダーを初期化
    let file = match File::open(target_file_abs) {
        Ok(file) => file,
        Err(_) => return Err(anyhow!("CSV file read error."))
    };
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);
    
    // ヘッダーを取得
    let headers = reader.headers()?
        .clone();

    // 対応するインデックス番号を取得
    let header_index: Vec<usize> = headers
        .iter()
        .enumerate()
        .filter_map(|(i, h)| if col_names.contains(h) { Some(i) } else { None})
        .collect();

    // 新しいヘッダーを作成
    let mut new_headers = headers.clone();
    new_headers.push_field(new_column_name);

    // 標準出力用のWriterを作成
    let mut writer = WriterBuilder::new().from_writer(io::stdout());
    writer.write_record(&new_headers)?;

    for result in reader.records() {
        let record = result?;
        let mut new_record = record.clone();

        let values: Vec<&str> = header_index
            .iter()
            .map(|&i| record.get(i).unwrap_or(""))
            .collect();

        let result_value = match mode {
            SumMode::Auto => {
                if values.iter().all(|v| v.parse::<f64>().is_ok()) {
                    let sum: f64 = values.iter().map(|v| v.parse::<f64>().unwrap()).sum();
                    sum.to_string()
                } else {
                    values.join("")
                }
            }
            SumMode::NumericOnly => {
                let sum: f64 = values
                    .iter()
                    .map(|v| v.parse::<f64>().map_err(|_| {
                        format!("")
                    }))
                    .collect::<Result<Vec<f64>, _>>()
                    .map_err(|e| anyhow!("Error: {}", e))?
                    .iter()
                    .sum();
                sum.to_string()
            }
            SumMode::ConcatOnly => values.join(""),
        };

        new_record.push_field(&result_value);
        writer.write_record(&new_record)?;
    }
    // ファイルをフラッシュして書き込みを確定
    writer.flush().expect("Flush error.");

    Ok("Complated.".to_string())
}