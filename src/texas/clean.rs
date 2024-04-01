use std::fs::File;
use std::io::{self, BufRead, Write};
use regex::Regex;
use anyhow::{Result, anyhow};
use super::super::utils::{
    get_abs_filepath,
    get_abs_directory_path,
    is_dir,
};

// 正規表現に一致する行を削除
pub fn clean_row(
    target_file: &str,
    regex_pattern: &str,
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

    let file = File::open(target_file_abs).map_err(|e| anyhow!("File open error.: {}", e))?;
    let reader = io::BufReader::new(file);

    // 正規表現を初期化
    let regex = Regex::new(regex_pattern).map_err(|e| anyhow!("Invalid regex pattern.: {}", e))?;

    // 出力ファイル名
    let output_path = output_directory_abs.join("clean_row.txt");

    let mut output = File::create(output_path).map_err(|e| anyhow!("File create error: {}", e))?;

    for line in reader.lines() {
        let line = line.map_err(|e| anyhow!("Line get error.: {}", e))?;
        if !regex.is_match(&line) {
            writeln!(output, "{}", line).map_err(|e| anyhow!("Write error.: {}", e))?;
        }
    }
    Ok("Complated".to_string())
}