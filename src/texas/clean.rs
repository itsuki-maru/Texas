use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use regex::Regex;
use anyhow::{Result, anyhow};
use super::super::utils::get_abs_filepath;

// 正規表現に一致する行を削除
pub fn clean_row(
    target_file: &str,
    regex_pattern: &str,
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

    let file = File::open(target_file_abs).map_err(|e| anyhow!("File open error.: {}", e))?;
    let reader = io::BufReader::new(file);

    // 正規表現を初期化
    let regex = Regex::new(regex_pattern).map_err(|e| anyhow!("Invalid regex pattern.: {}", e))?;

    // 標準出力用のWriterを作成
    let mut writer = BufWriter::new(io::stdout());

    for line in reader.lines() {
        let line = line.map_err(|e| anyhow!("Line get error.: {}", e))?;
        if !regex.is_match(&line) {
            writeln!(writer, "{}", line).map_err(|e| anyhow!("Write error.: {}", e))?;
        }
    }
    writer.flush().map_err(|e| anyhow!("Flush error.: {}", e))?;
    Ok("Complated".to_string())
}