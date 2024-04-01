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