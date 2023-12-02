fn main() {
    let data = String::from("1hello2\n3world24");
    let result = process_input(&data);
    println!("{}", result);
}

fn process_input(input: &str) -> usize {
    if input.is_empty() {
        return 0;
    }

    let words: Vec<&str> = input.trim().split('\n').collect();

    let mut result = 0;
    for word in words {
        result += get_decoded_number(word);
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
