use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::Write;

use crate::scheme::StatusData;

fn get_abs_path(filename: String) -> String {
    println!("{}", filename);
    filename
}

fn get_path_exists(abs_filepath: String) -> Option<bool> {
    println!("{}", abs_filepath);
    Some(true)
}

pub fn split_file(
    target_file: &str,
    regex_pattrern: &str
) -> StatusData {

    let abs_target_file_path = get_abs_path(target_file.to_string());
    println!("{}", abs_target_file_path);

    let is_exists = get_path_exists(abs_target_file_path);

    let text = fs::read_to_string(target_file).unwrap();
    let lines = text.split("\n").collect::<Vec<_>>();

    let mut index_list: Vec<u32> = Vec::new();
    let re = Regex::new(&regex_pattrern).unwrap();
    for (i, line) in lines.iter().enumerate() {
        if re.is_match(line) {
            index_list.push(i.try_into().unwrap());
        }
    }
    index_list.push(lines.len().try_into().unwrap());

    let mut data_range: Vec<u32> = Vec::new();
    let mut i = 1;
    for index in index_list {
        let output = format!("./output_{}.txt", i);
        data_range.push(index);
        if data_range.len() == 2 {
            let first: u32 = *data_range.first().unwrap();
            let last: u32 = *data_range.last().unwrap();
            let result = &lines[first as usize..last as usize];
            let write_data = result.join("\n");

            let mut f = File::create(output).unwrap();
            let bytes = write_data.as_bytes();
            f.write_all(bytes).unwrap();

            let tmp_index: u32 = *data_range.last().unwrap();
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