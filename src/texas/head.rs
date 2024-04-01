use csv::ReaderBuilder;
use std::fs::{self, File};
use anyhow::{Result, anyhow};
use super::super::utils::{
    get_abs_filepath,
    is_file,
};

// テキストファイルの先頭行かCSVファイルのヘッダー行を出力
pub fn print_head(
    target_file: &str,
    read_limit: usize,
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

    // テキストファイルを読み込み
    let text = match fs::read_to_string(&target_file_abs) {
        Ok(content) => content,
        Err(e) => return Err(anyhow!("File read error.: {}", e))
    };

    // 行を格納するベクタ
    let lines = text.split("\n").collect::<Vec<_>>();
    // 行番号
    let mut row_number = 1;
    for (i, line) in lines.iter().enumerate() {
        println!("{}:\t{}", row_number, line);
        row_number += 1;
        if i == read_limit-1 {
            break;
        }
    }
    Ok(())
}

// CSVファイルのヘッダー行を出力
pub fn print_header_csv(
    target_file: &str,
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
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    // ヘッダーを取得
    let headers = reader.headers()
        .map_err(|e| anyhow!("Header get error: {}", e))?
        .clone();

    for (i, header) in headers.iter().enumerate() {
        println!("{}:\t{}", i ,header);
    }
    Ok(())
}