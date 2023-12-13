use regex::Regex;
use std::fs::read_to_string;

pub fn problem_1() -> u32 {
    let mut result = 0;
    let regex = Regex::new(r"\b\d+\b").unwrap();
    let content = read_to_string("./src/inputs/day_4.1.txt").unwrap();
    let lines = content.lines();

    for line in lines {
        let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();

        let winning_numbers: Vec<u32> = regex
            .find_iter(parts[0])
            .map(|m| m.as_str().parse().unwrap())
            .collect();

        let player_numbers: Vec<u32> = regex
            .find_iter(parts[1])
            .map(|m| m.as_str().parse().unwrap())
            .collect();

        let mut matches = 0;
        for player_number in &player_numbers {
            if winning_numbers[1..].contains(player_number) {
                matches += 1;
            }
        }

        if matches > 0 {
            result += 2_u32.pow(matches - 1);
        }
        result += 0;
    }

    return result;
}

pub fn problem_2() -> u32 {
    let mut result = 0;
    let regex = Regex::new(r"\b\d+\b").unwrap();
    let content = read_to_string("./src/inputs/day_4.1.txt").unwrap();
    let lines = content.lines();

    for line in lines {
        let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();

        let winning_numbers: Vec<u32> = regex
            .find_iter(parts[0])
            .map(|m| m.as_str().parse().unwrap())
            .collect();

        let player_numbers: Vec<u32> = regex
            .find_iter(parts[1])
            .map(|m| m.as_str().parse().unwrap())
            .collect();

        let mut matches = 0;
        for player_number in &player_numbers {
            if winning_numbers[1..].contains(player_number) {
                matches += 1;
            }
        }

        if matches > 0 {
            result += 2_u32.pow(matches - 1);
        }
        result += 0;
    }

    return result;
}
