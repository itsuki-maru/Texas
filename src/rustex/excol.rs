use csv::{ReaderBuilder, WriterBuilder};
use std::collections::HashSet;
use std::fs::File;
use crate::scheme::StatusData;
use super::super::utils::{
    get_abs_filepath,
    get_abs_directory_path,
    is_dir,
};

// CSVファイルの指定した列名のみを抽出
pub fn extract_column(
    target_file: &str,
    col_names: HashSet<&str>,
    output_directory: &str,
) -> StatusData {

    // 対象ファイルの絶対パスを取得
    let target_file_abs = match get_abs_filepath(target_file) {
        Ok(path) => path,
        Err(_) => return StatusData {
            status_code: 404,
            message: format!("{} is not found.", target_file),
        },
    };

    // ファイルの存在確認
    if !target_file_abs.exists() {
        return StatusData {
            status_code: 404,
            message: format!("{} is not found.", target_file),    
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

    // CSVファイルを読み込み
    let file = match File::open(target_file_abs) {
        Ok(file) => file,
        Err(_) => {
            return StatusData {
                status_code: 400,
                message: "File read error.".to_string(),
            };
        }
    };
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);
    
    // ヘッダーを取得し、対応するインデックス番号を取得
    let headers = reader.headers().expect("Header get error.").clone();
    let header_index: Vec<usize> = headers
        .iter()
        .enumerate()
        .filter_map(|(i, h)| if col_names.contains(h) { Some(i) } else { None})
        .collect();

    // 全ての列が削除対象であればエラーを返す
    if header_index.is_empty() {
        return  StatusData {
            status_code: 400,
            message: "All columns are marked for deletion".to_string()
        };
    }

    // 出力ファイル名
    let output_path = output_directory_abs.join("extract_column.csv");
    // CSVファイル出力用のWriterを作成
    let mut writer = WriterBuilder::new().from_path(output_path).expect("Writer create error");

    // 残す列のみを含むヘッダーを書き込む
    let selected_headers: Vec<&str> = header_index.iter().map(|&i| headers.get(i).expect("Header get error")).collect();
    writer.write_record(&selected_headers).expect("Selected header write error.");

    // 各行に対し、選択された列のみを含むようにフィルタをかけて出力
    for result in reader.records() {
        let record = result.expect("Record read error.");
        let selected_fields: Vec<&str> = header_index.iter().map(|&i| record.get(i).expect("Field get error.")).collect();
        writer.write_record(&selected_fields).expect("Selected field write error.");
    }

    // ファイルをフラッシュして書き込みを確定
    writer.flush().expect("Flush error.");

    StatusData {
        status_code: 200,
        message: "File read successfully.".to_string(),
    }
}