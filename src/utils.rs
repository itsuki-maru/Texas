use std::{env, fs};
use std::path::{PathBuf, Path};
use dirs;

// ファイルの絶対パスを取得する関数
pub fn get_abs_filepath(filename: &str) -> std::io::Result<PathBuf> {
    // ホームディレクトリを取得
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found.")),
    };

    let path = Path::new(filename);

    // パスがホームディレクトリからの相対パスであるかチェック
    if path.starts_with("~/") {
        // ホームディレクトリのパスに置換
        let without_tilde = path.strip_prefix("~/")
            .expect("Cound not strip tilde.");
        Ok(home_dir.join(without_tilde))
    } else {
        // 通常のパス処理
        let current_dir = env::current_dir()?;
        Ok(current_dir.join(path))
    }
}

// ディレクトリの絶対パスを取得する関数
pub fn get_abs_directory_path(dir_name: &str) -> std::io::Result<PathBuf> {
    // ホームディレクトリを取得
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found.")),
    };

    let path = Path::new(dir_name);

    // パスがホームディレクトリからの相対パスであるかチェック
    if path.starts_with("~/") {
        // ホームディレクトリのパスに置換
        let without_tilde = path.strip_prefix("~/")
            .expect("Cound not strip tilde.");
        Ok(home_dir.join(without_tilde))
    } else {
        // 通常のパス処理
        let current_dir = env::current_dir()?;
        Ok(current_dir.join(path))
    }
}

// パスがファイルか判定する関数
pub fn is_file<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path).map(|metadata| metadata.is_file()).unwrap_or(false)
}

// パスがディレクトリか判定する関数
pub fn is_dir<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path).map(|metadata| metadata.is_dir()).unwrap_or(false)
}

// 数字のフォーマット
pub fn format_with_connma(number: i64) -> String {
    let num_str = number.to_string();
    let mut result = String::new();
    // 数字を分解して配列にベクタに格納
    let chars: Vec<char> = num_str.chars().rev().collect();

    // 3桁ごとにカンマをベクタに追加
    for (i, ch) in chars.iter().enumerate() {
        if i % 3 == 0 && i != 0 {
            result.push(',');
        }
        result.push(*ch);
    }
    // 結合して返却
    result.chars().rev().collect()
}