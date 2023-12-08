use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let (instructions, map) = read_input("./input.txt");
    let mut counter = 0;
    let mut current =  add_starting_positions(map.keys().map(|k| k.to_owned()).collect());
    let mut current_least_time = vec![0; current.len()];
    let mut iterator = instructions.chars().into_iter().cycle();
    while current_least_time.iter().any(|iterations| *iterations == 0) {
        let action = iterator.next().unwrap();
        for (idx, curr) in current.iter_mut().enumerate() {
            if curr.ends_with("Z") && current_least_time[idx] == 0 {
                current_least_time[idx] = counter;
            }
            let sides = map.get(curr).unwrap();
            match action {
                'L' => *curr = sides.0.clone(),
                'R' => *curr = sides.1.clone(),
                _ => continue,
            };
        }
        counter += 1;
    }

    let final_sum: u64 = lcm_of_vector(current_least_time);
    println!("{}", final_sum);
}

fn add_starting_positions(keys: Vec<String>) -> Vec<String> {
    keys.iter().filter(|key| key.ends_with("A")).cloned().collect()
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
    let alphabetic_chars = line.chars().filter(|char| char.is_alphanumeric()).collect::<String>();
    (
        alphabetic_chars[0..3].to_string(),
        alphabetic_chars[3..6].to_string(),
        alphabetic_chars[6..9].to_string(),
    )
}


fn lcm_of_vector(numbers: Vec<u64>) -> u64 {
    if numbers.is_empty() {
        return 0;
    }
    let mut result = numbers[0];
    for &num in numbers.iter().skip(1) {
        result = (result * num) / gcd(result, num);
    }
    result
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}