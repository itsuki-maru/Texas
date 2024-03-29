use std::fs::File;
use std::io::{self, BufRead};
use anyhow::{Result, anyhow};
use super::super::utils::{
    get_abs_filepath,
    is_file,
};

// テキストファイルの先頭行かCSVファイルのヘッダー行を出力
pub fn get_last_row(
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

    // テキストファイルを読み込み
    let file = File::open(target_file_abs).map_err(|e| anyhow!("File open error.: {}", e))?;
    let reader = io::BufReader::new(file);

    // ファイルの行を反転してイテレート
    let last_line = reader.lines()
        .filter_map(|line| line.ok()) // エラーを無視して有効な行のみを取得
        .filter(|line| !line.trim().is_empty()) // 空行を無視
        .last(); // 最終行を取得

    match last_line {
        Some(line) => println!("{}", line),
        None => println!("File is empty or only contains whitespace."),
    }
    Ok(())
}