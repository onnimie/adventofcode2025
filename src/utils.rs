use std::fs;
use std::str::Lines;


pub fn read_file_to_string(path: &str) -> String {
    let contents = fs::read_to_string(path).unwrap();
    contents
}


pub fn read_filelines_to_vec(path: &str) -> Vec<String> {
    let contents: String = fs::read_to_string(path).unwrap();
    let lines: Lines<'_> = contents.lines();
    let mut ret: Vec<String> = Vec::new();

    for line in lines {
        ret.push(line.to_string());
    }
    ret
}