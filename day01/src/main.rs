use std::env;
use std::fs;

fn main() {
    let file_path: &str = "src\\input.txt";

    let input: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    for line in input.lines() {
        println!("{line}")
    }
}