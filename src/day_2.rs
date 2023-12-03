use regex::Regex;
use std::fs::read_to_string;

pub fn problem_1() -> i32 {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    let mut result = 0;

    for line in read_to_string("./src/inputs/day_2.1.txt").unwrap().lines() {
        let regex_id = Regex::new(r"Game \d+").unwrap();
        let game_id_portion = regex_id
            .find_iter(line)
            .map(|m| m.as_str())
            .collect::<Vec<&str>>()[0];
        let game_id = &game_id_portion[5..].parse::<i32>().unwrap();

        let regex_plays = Regex::new(r"(\d+)\s*([a-zA-Z]+)").unwrap();
        let plays: Vec<&str> = regex_plays.find_iter(line).map(|m| m.as_str()).collect();

        let mut curr_max_red = 0;
        let mut curr_max_green = 0;
        let mut curr_max_blue = 0;

        for i in 0..plays.len() {
            let play: Vec<&str> = plays[i].split_whitespace().collect();
            let ammount: i32 = play[0].parse().unwrap();
            let color = play[1];

            if color == "red" && ammount > curr_max_red {
                curr_max_red = ammount;
            }

            if color == "green" && ammount > curr_max_green {
                curr_max_green = ammount;
            }

            if color == "blue" && ammount > curr_max_blue {
                curr_max_blue = ammount;
            }
        }

        if MAX_RED >= curr_max_red && MAX_GREEN >= curr_max_green && MAX_BLUE >= curr_max_blue {
            result += game_id;
        }
    }

    return result;
}

pub fn problem_2() -> i32 {
    let mut result = 0;

    for line in read_to_string("./src/inputs/day_2.2.txt").unwrap().lines() {
        let regex_plays = Regex::new(r"(\d+)\s*([a-zA-Z]+)").unwrap();
        let plays: Vec<&str> = regex_plays.find_iter(line).map(|m| m.as_str()).collect();

        let mut curr_max_red = 0;
        let mut curr_max_green = 0;
        let mut curr_max_blue = 0;

        for i in 0..plays.len() {
            let play: Vec<&str> = plays[i].split_whitespace().collect();
            let ammount: i32 = play[0].parse().unwrap();
            let color = play[1];

            if color == "red" && ammount > curr_max_red {
                curr_max_red = ammount;
            }

            if color == "green" && ammount > curr_max_green {
                curr_max_green = ammount;
            }

            if color == "blue" && ammount > curr_max_blue {
                curr_max_blue = ammount;
            }
        }
        result += curr_max_blue * curr_max_green * curr_max_red;
    }

    return result;
}
