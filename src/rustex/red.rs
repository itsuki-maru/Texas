use regex::Regex;
use std::fs;

use crate::scheme::StatusData;
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
) -> StatusData {

    // 対象ファイルの絶対パスを取得
    let target_file_abs = match get_abs_filepath(target_file) {
        Ok(path) => path,
        Err(_) => {
            return StatusData {
                status_code: 404,
                message: format!("{} is not found.", target_file),
            }
        }
    };

    // ファイルの存在確認
    if !target_file_abs.exists() {
        return StatusData {
            status_code: 404,
            message: format!("{} is not found.", target_file),
        };
    }

    // 指定されたファイルがファイルであることを確認
    if !is_file(&target_file_abs) {
        return StatusData {
            status_code: 400,
            message: format!("{} is not csv file.", target_file),
        };
    }

    // 出力先の絶対パスを取得
    let output_directory_abs = match get_abs_directory_path(output_directory) {
        Ok(path) => path,
        Err(_) => return StatusData {
            status_code: 404,
            message: format!("{} is not found.", output_directory),
        },
    };

    // 出力先がディレクトリか確認
    if !is_dir(&output_directory_abs) {
        return StatusData {
            status_code: 400,
            message: format!("{} is not directory.", output_directory),    
        };
    }

    // テキストファイルを読み込み
    let text_content = match fs::read_to_string(&target_file_abs) {
        Ok(content) => content,
        Err(_) => return StatusData {
            status_code: 500,
            message: "Failed to read the file.".to_string(),
        },
    };

    // 正規表現に基づいて文字列を置換
    let re = Regex::new(regex_pattern).expect("Invalid regex pattern.");
    let result_content = re.replace_all(&text_content, replaced_text);

    // 出力
    let output_path = output_directory_abs.join("red_output.txx");
    fs::write(output_path, result_content.as_ref()).expect("Write error.");

    StatusData {
        status_code: 200,
        message: "Regex replaced complated.".to_string(),
    }
}