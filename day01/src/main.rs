use std::fs;

fn main() {
    let file_path: &str = "src\\input.txt";

    let input: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let mut total: u32 = 0;

    for line in input.lines() {
        let mut calibration_value: u32 = 0;

        for c in line.chars() {
            if c.is_ascii_digit() {
                calibration_value = (c as u32 - 48) * 10;
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                calibration_value += c as u32 - 48;
                break;
            }
        }
        
        total += calibration_value;
    }

    println!("{total}");
}