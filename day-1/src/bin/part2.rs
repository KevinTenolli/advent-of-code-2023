use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut number_hashmap: HashMap<String, i32> = HashMap::new();
    populate_hashmap(&mut number_hashmap);
    let mut result = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                result += get_line_sum(&ip, &number_hashmap);
            }
        }
    }
    println!("{}", result);
}

fn populate_hashmap(hashmap: &mut HashMap<String, i32>) {
    hashmap.insert("one".to_owned(), 1);
    hashmap.insert("two".to_owned(), 2);
    hashmap.insert("three".to_owned(), 3);
    hashmap.insert("four".to_owned(), 4);
    hashmap.insert("five".to_owned(), 5);
    hashmap.insert("six".to_owned(), 6);
    hashmap.insert("seven".to_owned(), 7);
    hashmap.insert("eight".to_owned(), 8);
    hashmap.insert("nine".to_owned(), 9);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_line_sum(line: &str, map: &HashMap<String, i32>) -> i32 {
    let chars = vec!['o', 'e', 't', 'n'];
    let mut queue: VecDeque<i32> = VecDeque::new();
    let mut word_to_number = String::from("");
    for (_, ch) in line.chars().enumerate() {
        if ch.is_numeric() {
            queue.push_back(ch.to_string().parse().unwrap());
            word_to_number = String::from("");
        } else {
            word_to_number.push(ch);
            let map_keys = map.keys().into_iter();
            for key in map_keys {
                if word_to_number.contains(key) {
                    if let Some(&number) = map.get(key) {
                        queue.push_back(number);
                    }
                    let last_char = word_to_number.chars().last().unwrap();
                    if chars.contains(&last_char) {
                        word_to_number = String::from(last_char);
                    } else {
                        word_to_number = String::from("");
                    }
                }
            }
        }
    }
    match queue.len() {
        0 => 0,
        1 => {
            let val = queue.pop_back().unwrap();
            val * 10 + val
        }
        _ => queue.pop_front().unwrap() * 10 + queue.pop_back().unwrap(),
    }
}
