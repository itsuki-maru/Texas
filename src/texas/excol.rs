use csv::{ReaderBuilder, WriterBuilder};
use std::collections::HashSet;
use std::fs::File;
use std::io;
use anyhow::{Result, anyhow};
use super::super::utils::get_abs_filepath;

// CSVファイルの指定した列名のみを抽出してファイルに保存
pub fn extract_column(
    target_file: &str,
    col_names: HashSet<&str>,
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

    // 全ての列が削除対象であればエラーを返す
    if header_index.is_empty() {
        return Err(anyhow!("Mistake designation index."))
    }

    // CSVファイル出力用のWriterを作成
    let mut writer = WriterBuilder::new().from_writer(io::stdout());

    // 残す列のみを含むヘッダーを書き込む
    let selected_headers: Vec<&str> = header_index.iter().map(|&i| headers.get(i).expect("Header get error")).collect();
    writer.write_record(&selected_headers).map_err(|e| anyhow!("Selected header write error.: {}", e))?;

    // 各行に対し、選択された列のみを含むようにフィルタをかけて出力
    for result in reader.records() {
        let record = result.expect("Record read error.");
        let selected_fields: Vec<&str> = header_index.iter().map(|&i| record.get(i).expect("Field get error.")).collect();
        writer.write_record(&selected_fields).expect("Selected field write error.");
    }

    // ファイルをフラッシュして書き込みを確定
    writer.flush().expect("Flush error.");

    Ok("Complated.".to_string())
}