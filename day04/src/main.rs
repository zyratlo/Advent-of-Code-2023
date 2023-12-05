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
    // Keeps track of the numbers of copies made of every card
    let mut copies: Vec<u32> = vec![1; input.lines().count()];

    // Iterate through every line in the input
    for (index, line) in input.lines().enumerate() {
        // Grab the winning numbers and my numbers
        let winning_nums: Vec<&str> = (&line[line.chars().position(|c| c == ':').unwrap()+1..line.chars().position(|c| c == '|').unwrap()]).trim().split_whitespace().collect();
        let my_nums: Vec<&str> = (&line[line.chars().position(|c| c == '|').unwrap()+1..line.len()]).trim().split_whitespace().collect();

        let mut matches: u32 = 0;
        
        // Count the number of matches
        for num in my_nums {
            if winning_nums.contains(&num) {
                matches += 1;
            }
        }

        // Iterate through every copy of the card
        for _i in 0..copies[index] {
            total += 1;

            // Add copies to the necessary following cards
            for i in 1..matches+1 {
                copies[index + i as usize] += 1;
            }
        }
    }
    println!("Total scratchcards: {total}");
}


fn part1(input: &str) {
    let mut total = 0;

    // Iterate through every line in input
    for line in input.lines() {
        // Grab the winning numbers and my numbers
        let winning_nums: Vec<&str> = (&line[line.chars().position(|c| c == ':').unwrap()+1..line.chars().position(|c| c == '|').unwrap()]).trim().split_whitespace().collect();
        let my_nums: Vec<&str> = (&line[line.chars().position(|c| c == '|').unwrap()+1..line.len()]).trim().split_whitespace().collect();

        let mut matches: i32 = -1;
        let base: u32 = 2;

        // Count the number of matches
        for num in my_nums {
            if winning_nums.contains(&num) {
                matches += 1;
            }
        }

        // Calcalute the score according to the number of matches
        if matches > -1 {
            total += base.pow(matches as u32);
        }
    }
    println!("Total points: {total}");
}