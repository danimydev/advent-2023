use regex::Regex;
use std::fs::read_to_string;

struct Part {
    text: String,
    irow: usize,
    icol: usize,
}

impl Part {
    fn int(&self) -> i32 {
        self.text.parse().unwrap()
    }
}

pub fn problem_1() -> i32 {
    let content = read_to_string("./src/inputs/day_3.1.txt").unwrap();
    let input = &content;
    let rows: Vec<&str> = input.lines().collect();

    let symbols = parse(&rows, &Regex::new(r#"[^.0-9]"#).unwrap());
    let nums = parse(&rows, &Regex::new(r#"\d+"#).unwrap());

    nums.into_iter()
        .filter(|n| symbols.iter().any(|s| adjacent(s, &n)))
        .map(|n| n.int())
        .sum()
}

pub fn problem_2() -> i32 {
    let content = read_to_string("./src/inputs/day_3.2.txt").unwrap();
    let input = &content;
    let rows: Vec<&str> = input.lines().collect();
    
    let gears = parse(&rows, &Regex::new(r#"\*"#).unwrap());
    let numbers = parse(&rows, &Regex::new(r#"\d+"#).unwrap());

    gears.into_iter()
        .filter(|g| {
            let neighbours: Vec<i32> = numbers.iter().filter(|&n| adjacent(n, &g)).map(|n| n.int()).collect();
            neighbours.len() == 2
        })
        .map(|g| {
            let neighbours: Vec<i32> = numbers.iter().filter(|&n| adjacent(n, &g)).map(|n| n.int()).collect();
            neighbours[0] * neighbours[1]
        })
        .sum()
}

fn parse(rows: &[&str], rx: &Regex) -> Vec<Part> {
    let mut result = Vec::new();
    for irow in 0..rows.len() {
        for mat in rx.find_iter(rows[irow]) {
            result.push(Part {
                text: mat.as_str().to_string(),
                irow,
                icol: mat.start(),
            });
        }
    }
    result
}

fn adjacent(p1: &Part, p2: &Part) -> bool {
    (p2.irow as isize - p1.irow as isize).abs() <= 1
        && p1.icol <= p2.icol + p2.text.len()
        && p2.icol <= p1.icol + p1.text.len()
}
