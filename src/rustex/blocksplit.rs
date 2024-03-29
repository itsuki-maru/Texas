use csv::ReaderBuilder;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use anyhow::{Result, anyhow};
use super::super::utils::{
    get_abs_filepath,
    get_abs_directory_path,
    is_dir,
    is_file,
};


// CSVファイルを指定した列でブロックに分割し、それぞれを別のファイルに出力
pub fn block_split(
    target_file: &str,
    target_column: &str,
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

    // CSVリーダーを初期化
    let file = match File::open(target_file_abs) {
        Ok(file) => file,
        Err(_) => return Err(anyhow!("CSV file read error."))
    };
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);
    // ヘッダーを取得
    let headers = reader.headers().expect("Header get error.").clone();

    // target_columnのインデックス番号を見つける
    let target_column_index = headers
        .iter()
        .position(|h| h == target_column)
        .ok_or_else(|| format!("Column name: `{}` not found.", target_column))
        .map_err(|e| anyhow!("Column index get error: {}", e))?;

    // 一時的にテキストデータを保持するベクタ
    let mut tmp_text_data: Vec<String> = Vec::new();
    // 現在処理中の値を追跡する変数
    let mut tmp_check_value: String = String::new();
    // 出力ファイルの名前に使用するカウンタ
    let mut file_name_number: usize = 0;
    // 最初の行を特別扱いするフラグ
    let mut first_row = true;

    for result in reader.records() {
        let record = result.expect("Record get error.");
        // 最初の行の処理：追跡する値を設定し、テキストデータに追加
        if first_row {
            tmp_check_value = record.get(target_column_index).unwrap_or_default().to_string();
            tmp_text_data.push(record.iter().collect::<Vec<&str>>().join(","));
            first_row = false;
            continue;
        }

        // 現在の行の指定された列が前の値と一致する場合、テキストデータに追加
        if record.get(target_column_index).unwrap_or_default() == tmp_check_value {
            tmp_text_data.push(record.iter().collect::<Vec<&str>>().join(","));
            continue;
        } else {
            // 値が変わった場合、新しいファイルを作成してテキストデータを書き込む
            file_name_number += 1;
            let output_filename = format!("output/block_split_{}.csv", file_name_number);
            let base_path = Path::new(&output_filename).parent().expect("Get base path error.");
            fs::create_dir_all(base_path).expect("parent directory create error.");

            let mut file = File::create(&output_filename).expect("File create error.");
            // ヘッダーを最初に書き込む
            writeln!(file, "{}", headers.iter().collect::<Vec<&str>>().join(",")).expect("Header write error.");
            // 次にテキストデータを書き込む
            writeln!(file, "{}", tmp_text_data.join("\n")).expect("Row write error.");

            // テキストデータと追跡する値をリセット
            tmp_text_data.clear();
            tmp_check_value = record.get(target_column_index).unwrap_or_default().to_string();
            tmp_text_data.push(record.iter().collect::<Vec<&str>>().join(","));
        }
    }

    // 最後のデータブロックを処理
    if !tmp_text_data.is_empty() {
        file_name_number += 1;
        let output_filename = format!("output/block_split_{}.csv", file_name_number);
        let base_path = Path::new(&output_filename).parent().expect("Get base path error.");
        fs::create_dir_all(base_path).map_err(|e| anyhow!("Parent directory create error.: {}", e))?;

        let mut file = File::create(&output_filename).map_err(|e| anyhow!("File create error.: {}", e))?;
        // ヘッダーを最初に書き込む
        writeln!(file, "{}", headers.iter().collect::<Vec<&str>>().join(",")).map_err(|e| anyhow!("Header write error.: {}", e))?;
        // 次にテキストデータを書き込む
        writeln!(file, "{}", tmp_text_data.join("\n")).map_err(|e| anyhow!("Row write error.: {}", e))?;
    }

    Ok("Complated.".to_string())
}