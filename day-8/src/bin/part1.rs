use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let (instructions, map) = read_input("./input.txt");
    let end = String::from("ZZZ");
    let mut counter = 0;
    let mut current = String::from("AAA");

    let mut iterator = instructions.chars().into_iter().cycle();

    while current != end {
        let action = iterator.next().unwrap();
        let sides = map.get(&current).unwrap();

        match action {
            'L' => current = sides.0.clone(),
            'R' => current = sides.1.clone(),
            _ => continue,
        };

        counter += 1;
    }

    println!("{}", counter);
}

fn read_input(filename: &str) -> (String, BTreeMap<String, (String, String)>) {
    let mut instructions = String::new();
    let mut map: BTreeMap<String, (String, String)> = BTreeMap::new();

    if let Ok(file) = File::open(filename) {
        let reader = io::BufReader::new(file);

        for (idx, line_result) in reader.lines().enumerate() {
            if let Ok(line) = line_result {
                if idx == 0 {
                    instructions = line;
                } else if !line.is_empty() {
                    let data = parse_line(&line);
                    map.insert(data.0, (data.1, data.2));
                }
            }
        }
    }

    (instructions, map)
}

fn parse_line(line: &str) -> (String, String, String) {
    let alphabetic_chars = line.chars().filter(|char| char.is_alphabetic()).collect::<String>();
    (
        alphabetic_chars[0..3].to_string(),
        alphabetic_chars[3..6].to_string(),
        alphabetic_chars[6..9].to_string(),
    )
}
