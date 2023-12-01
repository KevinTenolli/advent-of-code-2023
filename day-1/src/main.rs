use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut result = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                result += get_line_sum(&ip);
            }
        }
    }
    println!("{}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_line_sum(line: &str) -> i32 {
    let mut queue: VecDeque<i32> = VecDeque::new();
    for (_, ch) in line.chars().enumerate() {
        if ch.is_numeric() {
            queue.push_front(ch.to_string().parse().unwrap());
        }
    }

    match queue.len() {
        0 => 0,
        1 => {
            let val = queue.pop_back().unwrap();
            val * 10 + val
        }
        _ => queue.pop_back().unwrap() * 10 + queue.pop_front().unwrap(),
    }
}
