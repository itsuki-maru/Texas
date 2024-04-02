use csv::ReaderBuilder;
use std::collections::HashMap;
use std::fs::File;
use num_format::{Locale, ToFormattedString};
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
    let headers = reader.headers()?
        .clone();

    // key_columnのインデックスを見つける
    let key_column_index = headers
        .iter()
        .position(|h| h == key_column)
        .ok_or_else(|| anyhow!("Column name: `{}` not found.", key_column))?;

    // 計算対象の列のインデックスを事前に計算
    let target_columns_indexes = target_columns.iter()
        .map(|&column| {
            headers.iter().position(|h| h == column)
                .ok_or_else(|| anyhow!("Column name: `{}` not found.", column))
        })
        .collect::<Result<Vec<_>, _>>()?;

    // 各キーに対する合計と件数を保持するHashMap
    let mut data: HashMap<String, HashMap<String, (f64, u32)>> = HashMap::new();

    // レコードをイテレート
    for result in reader.records() {
        let record = result?;
        let key = record.get(key_column_index).unwrap_or_default().to_string();

        // 集計対象の列番号の数だけ繰り返す
        for column in &target_columns_indexes {
            // レコードから列番号の値を取得（&usize（usizeの参照）をusize（値そのもの）に参照の内容をデリファレンス（参照外し））
            let value_str = record.get(*column).unwrap_or("0");
            // 小数点モードか否かでパースする値を変更
            let value = if floatmode {
                value_str.parse::<f64>().unwrap_or(0.0)
            } else {
                value_str.parse::<f64>().unwrap_or(0.0) as f64
            };

            // data.entryで集計対象の現在の合計と出現回数を取り出すが集計対象がHashMapに存在しなければ新たなHashMap{}を作成
            let entry = data.entry(column.to_string()).or_insert_with(HashMap::new);
            // 集計対象の現在の合計と出現回数が空であれば(0.0, 0)を挿入
            let counts = entry.entry(key.clone()).or_insert((0.0, 0));

            // 集計対象の現在の合計と出現回数に加算
            counts.0 += value;
            counts.1 += 1;
        }

    }

    // 結果を出力
    if is_csv {
        // CSV形式で出力
        for case in target_columns_indexes.iter() {
            let key_column = match headers.get(*case) {
                Some(col_name) => col_name,
                None => return Err(anyhow!("Column name: `{}` not found.", key_column)),
            };
            println!("========== {} ==========", key_column);
            println!("CASE,TOTAL,COUNT,AVE");
            if let Some(inner_map) = data.get(&case.to_string()) {
                for item in inner_map.iter() {
                    let average = item.1.0 / item.1.1 as f64;
                    println!("{},{},{},{}", item.0, item.1.0, item.1.1, average);
                }
            }
        }
    } else {
        // 標準出力
        for case in target_columns_indexes.iter() {
            let key_column = match headers.get(*case) {
                Some(col_name) => col_name,
                None => return Err(anyhow!("Column name: `{}` not found.", key_column)),
            };
            println!("====================== KEY COLUMN: {} ======================", key_column);
            if let Some(inner_map) = data.get(&case.to_string()) {
                for item in inner_map.iter() {
                    // total(f64)のフォーマット
                    let total_integet_part = item.1.0.trunc() as i64;
                    let total_decimal_part = item.1.0.fract();
                    let formatted_total_integer = total_integet_part.to_formatted_string(&Locale::en);
                    let formatted_total = format!("{}{}", formatted_total_integer, format!("{:.3}", total_decimal_part).trim_start_matches('0'));

                    // count(i32)のフォーマット
                    let count_formatted = item.1.1.to_formatted_string(&Locale::en);

                    // 平均値のフォーマット
                    let average = item.1.0 / item.1.1 as f64;
                    let average_integet_part = average.trunc() as i64;
                    let average_decimal_part = average.fract();
                    let formatted_average_integer = average_integet_part.to_formatted_string(&Locale::en);
                    let formatted_average = format!("{}{}", formatted_average_integer, format!("{:.3}", average_decimal_part).trim_start_matches('0'));

                    println!("CASE:\t{}\tTOTAL:\t{}\tCOUNT:\t{}\tAVE:\t{}", item.0, formatted_total, count_formatted, formatted_average);
                }
            }
        }
    }

    Ok(())
}
