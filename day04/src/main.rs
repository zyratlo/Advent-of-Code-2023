use std::fs;

fn main() {
    let file_path: &str = "src\\input.txt";

    let input: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // part1(&input);
    
    part2(&input);
}

fn part2(input: &str) {
    let mut total = 0;
    let mut copies: Vec<u32> = vec![1; input.lines().count()];

    for (index, line) in input.lines().enumerate() {
        let winning_nums: Vec<&str> = (&line[line.chars().position(|c| c == ':').unwrap()+1..line.chars().position(|c| c == '|').unwrap()]).trim().split_whitespace().collect();
        let my_nums: Vec<&str> = (&line[line.chars().position(|c| c == '|').unwrap()+1..line.len()]).trim().split_whitespace().collect();

        let mut matches: u32 = 0;

        for num in my_nums {
            if winning_nums.contains(&num) {
                matches += 1;
            }
        }
        for _i in 0..copies[index] {
            total += 1;
            for i in 1..matches+1 {
                copies[index + i as usize] += 1;
            }
        }
    }
    println!("Total scratchcards: {total}");
}


fn part1(input: &str) {
    let mut total = 0;

    for line in input.lines() {
        let winning_nums: Vec<&str> = (&line[line.chars().position(|c| c == ':').unwrap()+1..line.chars().position(|c| c == '|').unwrap()]).trim().split_whitespace().collect();
        let my_nums: Vec<&str> = (&line[line.chars().position(|c| c == '|').unwrap()+1..line.len()]).trim().split_whitespace().collect();

        let mut matches: i32 = -1;
        let base: u32 = 2;

        for num in my_nums {
            if winning_nums.contains(&num) {
                matches += 1;
            }
        }
        if matches > -1 {
            total += base.pow(matches as u32);
        }
    }
    println!("Total points: {total}");
}