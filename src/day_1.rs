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
    let mut first = String::new();
    let mut last = String::new();

    for c in word.chars() {
        if c.is_numeric() {
            first.push(c);
            break;
        }
    }

    for c in word.chars().rev() {
        if c.is_numeric() {
            last.push(c);
            break;
        }
    }

    let decoded_number_str = format!("{}{}", first, last);
    return decoded_number_str.parse().unwrap_or(0);
}

pub fn problem_2() -> usize {
    let mut result = 0;
    for line in read_to_string("./src/inputs/day_1.2.txt").unwrap().lines() {
        println!(
            "line: {} decoded: {}",
            line,
            get_decoded_number_with_words(line)
        );
        result += get_decoded_number_with_words(line);
    }
    return result;
}

fn get_decoded_number_with_words(word: &str) -> usize {
    let pattern = r"(?:zero|one|two|three|four|five|six|seven|eight|nine|ten|\d+)";
    let regex = Regex::new(pattern).unwrap();

    let matches: Vec<&str> = regex.find_iter(word).map(|m| m.as_str()).collect();
    println!("{:?}", matches);
    if matches.len() == 1 {
        let decoded = digit_name_to_int(matches[0]).unwrap();
        if decoded > 10 {
            let decoded_str = decoded.to_string();
            return format!(
                "{}{}",
                decoded_str.chars().nth(0).unwrap(),
                decoded_str.chars().nth(decoded_str.len() - 1).unwrap()
            )
            .parse()
            .unwrap();
        }
        return format!("{}{}", decoded, decoded).parse().unwrap_or(0);
    }

    let mut first_int: u32 = digit_name_to_int(matches[0]).unwrap();
    if first_int > 10 {
        first_int = format!("{}", first_int.to_string().chars().nth(0).unwrap(),)
            .parse()
            .unwrap()
    }

    let mut last_int: u32 = digit_name_to_int(matches[matches.len() - 1]).unwrap();
    if last_int > 10 {
        last_int %= 10;
    }

    let decoded_number_str = format!("{}{}", first_int, last_int);
    return decoded_number_str.parse().unwrap_or(0);
}

fn digit_name_to_int(digit_name: &str) -> Option<u32> {
    match digit_name.to_lowercase().as_str() {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => Some(digit_name.parse().unwrap_or(0)),
    }
}
