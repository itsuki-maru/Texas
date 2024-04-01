use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, Write};
use anyhow::{Result, anyhow};
use super::super::utils::{
    get_abs_filepath,
    get_abs_directory_path,
    is_dir,
    is_file,
};

// 正規表現にマッチする行のみを抽出してファイルへ出力
pub fn grep_row(
    target_file: &str,
    regex_pattern: &str,
    output_directory: &str,
    csv_header: bool,
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

    // 指定されたファイルがファイルであることを確認
    if !is_file(&target_file_abs) {
        return Err(anyhow!(format!("{} is not text file.", target_file)))
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

    // 正規表現パターンを初期化
    let regex = Regex::new(regex_pattern).map_err(|e| anyhow!("Regex init error: {}", e))?;

    // テキストファイルを読み込み
    let file = File::open(target_file).map_err(|e| anyhow!("File open error: {}", e))?;
    let reader = io::BufReader::new(file);

    // 出力ファイル名
    let output_path = output_directory_abs.join("grep.txt");
    let mut output = File::create(output_path).map_err(|e| anyhow!("File create error: {}", e))?;

    // 行を取得
    let mut lines = reader.lines().filter_map(Result::ok);
    // CSVのヘッダーを残す場合の処理
    if csv_header {
        if let Some(header) = lines.next() {
            writeln!(output, "{}", header).map_err(|e| anyhow!("CSV header write error.: {}", e))?;
        }
    }

    // 正規表現にマッチした行をWriterに書き込み
    for line in lines {
        if regex.is_match(&line) {
            writeln!(output, "{}", line).map_err(|e| anyhow!("Line write error: {}", e))?;
        }
    }
    Ok("Complated".to_string())
}