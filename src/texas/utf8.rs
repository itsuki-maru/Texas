use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use encoding_rs::SHIFT_JIS;
use anyhow::{Result, anyhow};
use super::super::utils::get_abs_filepath;

// 正規表現に一致する行を削除
pub fn shiftjis_to_utf8(
    target_file: &str,
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
    let mut reader = io::BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    // SHIFT-JISをUTF-8に変換
    let (decoded, _, had_errors) = SHIFT_JIS.decode(&buffer);
    if had_errors {
        return Err(anyhow!(format!("File read error: {}", target_file)))
    }

    // UTF8として書き出し
    // 標準出力用のWriterを作成
    let mut writer = BufWriter::new(io::stdout());
    writer.write_all(decoded.as_bytes())?;
    writer.flush().map_err(|e| anyhow!("Flush error.: {}", e))?;
    Ok("Complated".to_string())
}