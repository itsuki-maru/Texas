use std::fs::File;
use std::io::{self, BufRead};
use anyhow::{Result, anyhow};
use super::super::utils::{
    get_abs_filepath,
    is_file,
};

// テキストファイルの行数を取得
pub fn line_count(
    target_file: &str
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

    // ファイルをイテレートして行数を確認
    let line_count = reader.lines().count();

    println!("{}", line_count);

    Ok(())
}

// 文字数をカウント
pub fn word_count(
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

    // 文字数をカウント
    let mut char_count = 0;
    for line in reader.lines() {
        let line = line?; // io::Result<String>を取り出す
        char_count += line.chars().count(); // 行の文字数を加算
    }

    println!("{}", char_count);

    Ok(())
}