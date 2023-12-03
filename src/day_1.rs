use regex::Regex;
use std::fs::read_to_string;

pub fn problem_1() -> usize {
    let mut result = 0;
    for line in read_to_string("./src/inputs/day_1.1.txt").unwrap().lines() {
        result += get_decoded_number(line.trim());
    }
    return result;
}

fn get_decoded_number(word: &str) -> usize {
    let digit_regex = Regex::new(r"\d").unwrap();
    let matches: Vec<&str> = digit_regex.find_iter(&word).map(|m| m.as_str()).collect();
    return format!("{}{}", matches[0], matches[matches.len() - 1]).parse().unwrap();
}

pub fn problem_2() -> usize {
    let mut result = 0;
    for line in read_to_string("./src/inputs/day_1.2.txt").unwrap().lines() {
        result += get_decoded_number_with_words(line);
    }
    return result;
}

fn get_decoded_number_with_words(word: &str) -> usize {
    let digit_regex = Regex::new(r"\d").unwrap();
    let mut parsed = String::from(word);
    parsed = parsed.replace("one", "o1e");
    parsed = parsed.replace("two", "t2o");
    parsed = parsed.replace("three", "t3e");
    parsed = parsed.replace("four", "4");
    parsed = parsed.replace("five", "5e");
    parsed = parsed.replace("six", "6");
    parsed = parsed.replace("seven", "7n");
    parsed = parsed.replace("eight", "8");
    let parsed = parsed.replace("nine", "n9e");

    let matches: Vec<&str> = digit_regex.find_iter(&parsed).map(|m| m.as_str()).collect();

    return format!("{}{}", matches[0], matches[matches.len() - 1]).parse().unwrap();
}
