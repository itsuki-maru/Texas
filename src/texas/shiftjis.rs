use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use encoding_rs::SHIFT_JIS;
use anyhow::{Result, anyhow};
use super::super::utils::get_abs_filepath;

// 正規表現に一致する行を削除
pub fn utf8_to_shiftjis(
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
    let mut utf8_content = String::new();
    reader.read_to_string(&mut utf8_content)?;

    // UTF-8をSHIFT-JISに変換
    let (encoded, _, had_errors) = SHIFT_JIS.encode(&utf8_content);
    if had_errors {
        return Err(anyhow!(format!("File read error: {}", target_file)))
    }

    // SHIFT-JISとして書き出し
    // 標準出力用のWriterを作成
    let mut writer = BufWriter::new(io::stdout());
    writer.write_all(&encoded)?;
    writer.flush().map_err(|e| anyhow!("Flush error.: {}", e))?;
    Ok("Complated".to_string())
}