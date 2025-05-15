use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use anyhow::{Result, anyhow};
use super::super::utils::{
    get_abs_filepath,
    is_file,
};

// 正規表現にマッチする行のみを抽出してファイルへ出力
pub fn grep_row(
    target_file: &str,
    regex_pattern: &str,
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

    // 正規表現パターンを初期化
    let regex = Regex::new(regex_pattern).map_err(|e| anyhow!("Regex init error: {}", e))?;

    // テキストファイルを読み込み
    let file = File::open(target_file).map_err(|e| anyhow!("File open error: {}", e))?;
    let reader = io::BufReader::new(file);

    // 標準出力用のWriterを作成
    let mut writer = BufWriter::new(io::stdout());

    // 行を取得
    let mut lines = reader.lines().filter_map(Result::ok);
    // CSVのヘッダーを残す場合の処理
    if csv_header {
        if let Some(header) = lines.next() {
            writeln!(writer, "{}", header).map_err(|e| anyhow!("CSV header write error.: {}", e))?;
        }
    }

    // 正規表現にマッチした行をWriterに書き込み
    for line in lines {
        if regex.is_match(&line) {
            writeln!(writer, "{}", line).map_err(|e| anyhow!("Line write error: {}", e))?;
        }
    }
    writer.flush().map_err(|e| anyhow!("Flush error.: {}", e))?;
    Ok("Complated".to_string())
}