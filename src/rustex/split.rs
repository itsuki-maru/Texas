use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::Write;
use crate::scheme::StatusData;
use super::super::utils::{
    get_abs_filepath,
    get_abs_directory_path,
    is_dir,
};


// "split"
pub fn split_file(
    target_file: &str,
    regex_pattrern: &str,
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

    // テキストファイルを読み込み
    let text = match fs::read_to_string(&target_file_abs) {
        Ok(content) => content,
        Err(_) => return StatusData {
            status_code: 500,
            message: "Failed to read the file.".to_string(),
        },
    };

    let re = match Regex::new(&regex_pattrern) {
        Ok(r) => r,
        Err(_) => return StatusData {
            status_code: 500,
            message: "Invalid regex pattern.".to_string(),
        },
    };

    // 行を格納するベクタを定義
    let lines = text.split("\n").collect::<Vec<_>>();

    // lines.iter().enumerate(): linesベクタの各要素に対してイテレーション（繰り返し処理）を行い、enumerateメソッドを用いてそれぞれの要素にインデックス番号を付与
    // enumerateは各要素をタプル(i, line)の形で返します。ここでiは要素のインデックス（0から始まる）、lineはその要素の値（ここではファイルの各行のテキスト）
    // filter_mapメソッドは、イテレータの各要素に対して関数を適用し、その関数がSome(value)を返した場合のみ、valueを新しいイテレータの要素として収集
    // 正規表現reにマッチする行に対してのみ、その行のインデックス（i）をu32型にキャストしてSome(i as u32)として返す。マッチしない行はNoneを返し、その結果これらの行は無視
    let mut index_list: Vec<u32> = lines.iter().enumerate().filter_map(|(i, line)| {
        if re.is_match(line) {
            Some(i as u32)
        } else {
            None
        }
    }).collect(); // 最後に、filter_mapから返されるイテレータをcollectメソッドを使ってベクタに収集。この結果、index_listはマッチした行のインデックスのみを含むVec<u32>型のベクタとなる
    // index_listベクタにファイルの総行数（lines.len()をu32にキャストした値）を追加しています。これにより、後続の処理でファイルの最後のセクションを正しく処理するための終端インデックスとして機能
    index_list.push(lines.len() as u32);


    let mut data_range: Vec<u32> = Vec::new();
    let mut i = 1;
    for index in index_list {
        let output_filename = format!("./output_{}.txt", i);
        data_range.push(index);
        if data_range.len() == 2 {
            let first: u32 = *data_range.first().expect("First index missing");
            let last: u32 = *data_range.last().expect("Last index missing");
            let result = &lines[first as usize..last as usize];
            let write_data = result.join("\n");

            let output_path = output_directory_abs.join(&output_filename);
            let mut f = match File::create(output_path) {
                Ok(file) => file,
                Err(_) => return StatusData {
                    status_code: 500,
                    message: format!("Failed to create file: {}", output_filename),
                }
            };
            if let Err(_) = f.write_all(write_data.as_bytes()) {
                return StatusData {
                    status_code: 500,
                    message: format!("Failed to write to file: {}", output_filename),
                };
            }

            let tmp_index: u32 = *data_range.last().expect("Last index missing after write");
            data_range.clear();
            data_range.push(tmp_index);
            i += 1;
        }
    }
    StatusData {
        status_code: 200,
        message: "File read successfully.".to_string(),
    }
}