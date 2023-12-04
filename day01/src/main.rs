use std::fs;

fn main() {
    let file_path: &str = "src\\input.txt";

    let input: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    // part1(&input);

    part2(&input);    
}

fn part2(input: &str) {
    let mut total: u32 = 0;

    for line in input.lines() {
        let mut calibration_value: u32 = 0;

        // Find the first and last digit
        let mut last_digit: u32 = 0;
        for i in 0..line.len() {
            // println!("{}", &line[i..line.len()]);
            // println!("{}", part2helper_match_front(&line[i..line.len()]));
            let is_match = part2helper_match_front(&line[i..line.len()]);

            if is_match != -1 {
                if calibration_value == 0 {
                    calibration_value += is_match as u32 * 10;
                }
                last_digit = is_match as u32;
            }
        }

        total += calibration_value + last_digit;
    }
    println!("Part 2 Answer: {total}");
}

fn part2helper_match_front(to_match: &str) -> i32{
    // Return if first char is a digit 0-9
    if to_match.as_bytes()[0].is_ascii_digit() {
        return to_match.as_bytes()[0] as i32 - 48;
    }
    
    let digits = [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];
    
    for digit in digits {
        if to_match.starts_with(digit.0) {
            return digit.1;
        }
    }
    
    return -1;
}

fn part1(input: &str) {
    let mut total: u32 = 0;

    for line in input.lines() {
        let mut calibration_value: u32 = 0;

        // Find the first digit
        for c in line.chars() {
            if c.is_ascii_digit() {
                calibration_value = (c as u32 - 48) * 10;
                break;
            }
        }

        // Find the last digit
        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                calibration_value += c as u32 - 48;
                break;
            }
        }
        
        total += calibration_value;
    }

    println!("Part 1 Answer: {total}");
}