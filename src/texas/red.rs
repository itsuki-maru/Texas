use regex::Regex;
use std::fs;
use anyhow::{Result, anyhow};
use super::super::utils::{
    get_abs_directory_path,
    get_abs_filepath,
    is_file,
    is_dir,    
};

// テキストファイルの中から正規表現に一致する文字列を置換
pub fn red(
    target_file: &str,
    regex_pattern: &str,
    replaced_text: &str,
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

    // テキストファイルを読み込み
    let text_content = match fs::read_to_string(&target_file_abs) {
        Ok(content) => content,
        Err(e) => return Err(anyhow!("File read error.: {}", e))
    };

    // 正規表現に基づいて文字列を置換Invalid regex pattern.
    let re = Regex::new(regex_pattern).map_err(|e| anyhow!("Invalid regex pattern.: {}", e))?;
    let result_content = re.replace_all(&text_content, replaced_text);

    // 出力
    let output_path = output_directory_abs.join("red_output.txt");
    fs::write(output_path, result_content.as_ref()).map_err(|e| anyhow!("Write error.: {}", e))?;

    Ok("Complated".to_string())
}