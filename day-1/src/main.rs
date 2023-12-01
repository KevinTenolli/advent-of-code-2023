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
    let mut first = 0;
    let mut last = 0;
    for (_, ch) in line.chars().enumerate() {
        if ch.is_numeric() {
            if first == 0 {
                first = ch.to_string().parse().unwrap()
            }
            last = ch.to_string().parse().unwrap()
        }
    }
    first * 10 + last
}
