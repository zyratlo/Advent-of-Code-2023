use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path: &str = "src\\input.txt";

    let input: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    part1(&input);

    part2(&input);
}

fn part2(input: &str) {
    let mut total: u32 = 0;

    for game in input.lines() {
        let mut min_cubes:HashMap<&str, u32> = HashMap::new();

        // Iterate through every reveal
        for revealed in game[game.chars().position(|c| c == ':').unwrap()+1..game.len()].split(';') {
            // Iterate through every revealed color
            for revealed_cube in revealed.split(',') {
                let revealed_cube: Vec<&str> = revealed_cube.trim().split(' ').collect();
                let num: u32 = revealed_cube[0].parse().unwrap();
                let color: &str = revealed_cube[1];
                
                // Check if the number of revealed for that color is more than our constraint
                if !min_cubes.contains_key(color) || min_cubes[color] < num {
                    min_cubes.remove(color);
                    min_cubes.insert(color, num);
                }
            }
        }
        let mut temp_total: u32 = 1;
        for num in min_cubes.values() {
            temp_total *= num;
        }
        total += temp_total;
    }

    println!("Sum of powers: {total}");
}



fn part1(input: &str) {
    let mut total: u32 = 0;

    let mut constraints: HashMap<&str, u32> = HashMap::new();
    constraints.insert("red", 12);
    constraints.insert("green", 13);
    constraints.insert("blue", 14);


    for game in input.lines() {
        let mut possible: bool = true;

        // Grab the Game ID of the current line
        let game_id: u32 = (&game[5..game.chars().position(|c| c == ':').unwrap()]).parse().unwrap();
        
        // Iterate through every reveal
        for revealed in game[game.chars().position(|c| c == ':').unwrap()+1..game.len()].split(';') {
            // Iterate through every revealed color
            for revealed_cube in revealed.split(',') {
                let revealed_cube: Vec<&str> = revealed_cube.trim().split(' ').collect();
                let num: u32 = revealed_cube[0].parse().unwrap();
                let color: &str = revealed_cube[1];
                
                // Check if the number of revealed for that color is more than our constraint
                if constraints.get(color).unwrap() < &num {
                    possible = false;
                }
            }
        }

        if possible {
            total += game_id;
        }
    }

    println!("Sum of IDs of possible games: {total}");
}