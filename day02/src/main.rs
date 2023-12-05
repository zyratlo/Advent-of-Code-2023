use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path: &str = "src\\input.txt";

    let input: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    // let input: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    part1(&input);
}


fn part1(input: &str) {
    let mut total: u32 = 0;

    let mut constraints = HashMap::new();
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