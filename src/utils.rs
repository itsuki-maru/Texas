use std::{env, fs};
use std::path::{PathBuf, Path};

// ファイルの絶対パスを取得する関数
pub fn get_abs_filepath(filename: &str) -> std::io::Result<PathBuf> {
    let current_dir = env::current_dir()?;
    Ok(current_dir.join(filename))
}

// ディレクトリの絶対パスを取得する関数
pub fn get_abs_directory_path(dir_name: &str) -> std::io::Result<PathBuf> {
    let current_dir = env::current_dir()?;
    Ok(current_dir.join(dir_name))
}

// パスがファイルか判定する関数
pub fn is_file<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path).map(|metadata| metadata.is_file()).unwrap_or(false)
}

// パスがディレクトリか判定する関数
pub fn is_dir<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path).map(|metadata| metadata.is_dir()).unwrap_or(false)
}