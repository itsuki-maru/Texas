use csv::{ReaderBuilder, WriterBuilder};
use std::collections::HashSet;
use std::fs::File;
use anyhow::{Result, anyhow};
use super::super::utils::{
    get_abs_filepath,
    get_abs_directory_path,
    is_dir,
};

// CSVファイルの指定した列名のみを抽出してファイルに保存
pub fn extract_column(
    target_file: &str,
    col_names: HashSet<&str>,
    output_directory: &str,
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

    // 出力先の絶対パスを取得
    let output_directory_abs = match get_abs_directory_path(output_directory) {
        Ok(path) => path,
        Err(_) => return Err(anyhow!("Output directory absolute path get error."))
    };

    // 出力先がディレクトリか確認
    if !is_dir(&output_directory_abs) {
        return Err(anyhow!(format!("{} is not directory.", output_directory)))
    }

    // CSVリーダーを初期化
    let file = match File::open(target_file_abs) {
        Ok(file) => file,
        Err(_) => return Err(anyhow!("CSV file read error."))
    };
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);
    
    // ヘッダーを取得
    let headers = reader.headers()
        .map_err(|e| anyhow!("Header get error: {}", e))?
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

    // 出力ファイル名
    let output_path = output_directory_abs.join("extract_column.csv");
    // CSVファイル出力用のWriterを作成
    let mut writer = WriterBuilder::new().from_path(output_path).map_err(|e| anyhow!("Writer create error.: {}", e))?;

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