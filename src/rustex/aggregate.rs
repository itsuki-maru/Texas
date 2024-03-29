use csv::ReaderBuilder;
use std::collections::HashMap;
use std::fs::File;
use anyhow::{Result, anyhow};
use super::super::utils::{get_abs_filepath, is_file};


// CSVファイルを集計
pub fn aggregate_csv_data(
    target_file: &str,
    key_column: &str,
    target_columns: &[&str],
    floatmode: bool,
    is_csv: bool,
) -> Result<()> {
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

    // CSVファイルを読み込み
    let file = match File::open(target_file_abs) {
        Ok(file) => file,
        Err(_) => return Err(anyhow!("CSV file read error."))
    };
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    // ヘッダーを取得
    let headers = reader.headers()
        .map_err(|e| anyhow!("Header get error: {}", e))?
        .clone();

    // key_columnのインデックスを見つける
    let key_column_index = headers
        .iter()
        .position(|h| h == key_column)
        .ok_or_else(|| format!("Column name: `{}` not found.", key_column))
        .map_err(|e| anyhow!("Column index get error: {}", e))?;

    // 各キーに対する合計と件数を保持するHashMap
    let mut data: HashMap<String, HashMap<String, (f64, u32)>> = HashMap::new();

    // レコードをイテレート
    for result in reader.records() {
        let record = result.expect("CSV Error.");
        let key = record.get(key_column_index).unwrap_or_default();

        // ここでtarget_columnsは&strのスライスと仮定
        for &column in target_columns {
            // columnに対応するHashMapがなければ、新しく作成
            data.entry(column.to_string()).or_insert_with(HashMap::new);

            // ここで再度`get_mut`を呼び出す必要がある
            let column_map = data.get_mut(column).expect("Just inserted, should exist");

            let column_index = headers
                .iter()
                .position(|h| h == column)
                .expect(&format!("Column name: `{}` not found.", column));

            if let Some(value_str) = record.get(column_index) {
                let value = if floatmode {
                    value_str.parse::<f64>().unwrap_or(0.0)
                } else {
                    value_str.parse::<f64>().unwrap_or(0.0) // Rustではintとfloatの区別が厳密なため
                };

                let entry = column_map.entry(key.to_string()).or_insert((0.0, 0));
                entry.0 += value;
                entry.1 += 1;
            }
        }
    }

    // 結果を出力
    if is_csv {
        // CSV形式で出力
        println!("CASE,TOTAL,COUNT,AVE");
        for &column in target_columns {
            for (key, (sum, count)) in data.get(column).unwrap().iter() {
                // 平均を算出
                let average = sum / *count as f64;
                println!("{},{},{},{}", key, sum, count, average);
            }
        }
    } else {
        // 標準出力
        for &column in target_columns {
            println!("====================== KEY COLUMN: {} ======================", column);
            for (key, (sum, count)) in data.get(column).unwrap().iter() {
                // 平均を算出
                let average = sum / *count as f64;
                println!("CASE:\t{}\tTOTAL:\t{}\tCOUNT:\t{}\tAVE:\t{}", key, sum, count, average);
            }
        }
    }

    Ok(())
}
