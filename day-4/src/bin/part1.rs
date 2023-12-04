use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("./input.txt").expect("Error opening file");
    let reader = io::BufReader::new(file);
    let mut total_points = 0;
    for (_, line_result) in reader.lines().enumerate() {
        if let Ok(line) = line_result {
            let mut card_iterator = line.split(":").nth(1).unwrap().split("|").into_iter();
            total_points += get_line_result(card_iterator.next().unwrap(), card_iterator.next().unwrap());
        }
    }
    println!("{}", total_points);
}

fn get_line_result (card_numbers: &str, winning_numbers: &str) -> u32 {
    let mut points:u32 = 0;
    let mut winning_numbers_set: HashSet<u32> = HashSet::new();
    winning_numbers.split(" ").for_each(|number| {
        let parsed_number = parse_number(number);
        if parsed_number != 0 {
            winning_numbers_set.insert(parsed_number);
        }
    });
    card_numbers.split(" ").for_each(|number| {
        let parsed_number = parse_number(number);
        if winning_numbers_set.contains(&parsed_number) {
           if points == 0 {
               points = 1;
           } else {
               points *= 2;
           }
        }
    });
    points
}
fn parse_number(stringed_number: &str) -> u32 {
    stringed_number.parse::<u32>().unwrap_or(0)
}