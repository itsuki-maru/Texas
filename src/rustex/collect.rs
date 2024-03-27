use std::fs;
use std::path::Path;
use regex::Regex;
use glob::glob;
use super::super::utils::{
    get_abs_directory_path,
    is_dir,
};

use crate::scheme::StatusData;

// 正規表現にマッチする行を含むテキストファイルを収集
pub fn collect_file(
    target_dir: &str,
    destination_dir: &str,
    regex_pattern: &str,
) -> StatusData {

    let regex = Regex::new(regex_pattern).expect("Regex parttern error.");

    // ターゲットディレクトリの絶対パスを取得
    let target_directory_abs = match get_abs_directory_path(target_dir) {
        Ok(path) => path,
        Err(_) => return StatusData {
            status_code: 404,
            message: format!("{} is not found.", target_dir),
        },
    };

    // ターゲットディレクトリの存在確認
    if !target_directory_abs.exists() {
        return StatusData {
            status_code: 404,
            message: format!("{} is not found.", target_dir),
        }
    }

    // ターゲットディレクトリがディレクトリか確認
    if !is_dir(&target_directory_abs) {
        return StatusData {
            status_code: 400,
            message: format!("{} is not directory.", target_dir),    
        };
    }

    // 出力先の絶対パスを取得
    let destination_directory_abs = match get_abs_directory_path(destination_dir) {
        Ok(path) => path,
        Err(_) => return StatusData {
            status_code: 404,
            message: format!("{} is not found.", destination_dir),
        },
    };

    // 出力先の存在確認
    if !destination_directory_abs.exists() {
        return StatusData {
            status_code: 404,
            message: format!("{} is not found.", destination_dir),
        }
    }

    // 出力先がディレクトリか確認
    if !is_dir(&destination_directory_abs) {
        return StatusData {
            status_code: 400,
            message: format!("{} is not directory.", destination_dir),    
        };
    }

    // ターゲットディレクトリ内の.txtを検索
    let pattern = format!("{}/*.txt", target_dir);
    for entry in glob(&pattern).expect("Pattern error.") {
        match entry {
            Ok(path) => {
                if let Some(file_name) = path.file_name() {
                    let content = fs::read_to_string(&path).expect("Content read error.");
                    if regex.is_match(&content) {
                        let dest_path = Path::new(&destination_directory_abs).join(file_name);
                        fs::copy(&path,&dest_path).expect("Copy error.");

                    }
                }
            },
            Err(e) => return StatusData {
                status_code: 500,
                message: format!("{}", e),
            }
        }
    }
    StatusData {
        status_code: 200,
        message: "File collected.".to_string()
    }
}