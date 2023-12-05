use std::fs;

fn main() {
    let file_path: &str = "src\\input.txt";

    let input: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    part1(&input);
}


fn part1(input: &str) {
    let winning_nums: Vec<&str> = (&input[input.chars().position(|c| c == ':').unwrap()+1..input.chars().position(|c| c == '|').unwrap()]).trim().split_whitespace().collect();
    let my_nums: Vec<&str> = (&input[input.chars().position(|c| c == '|').unwrap()+1..input.len()]).trim().split_whitespace().collect();
}