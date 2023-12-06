use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let mut result = 1;
    if let Ok(file) = File::open("./input.txt") {
        let reader = io::BufReader::new(file);
        let mut line_iter = reader.lines();
        let time_string = line_iter.next().unwrap().unwrap();
        let distance_string = line_iter.next().unwrap().unwrap();
        let time_list = get_numbers(&time_string);
        let distance_list = get_numbers(&distance_string);
        for (time, dist) in time_list.iter().zip(distance_list.iter()) {
            result *= calculate_bounds(*time, *dist);
        }
    }
    dbg!(result);
}

fn get_numbers(stringed_numbers: &str) -> Vec<u32> {
    let mut numbers_list:Vec<u32> = Vec::new();
    stringed_numbers.split(":").nth(1).unwrap().split_whitespace().for_each(|number| {
        if let Ok(parsed_number) = number.parse::<u32>() {
            numbers_list.push(parsed_number);
        }
    });
    numbers_list
}

fn calculate_bounds(time: u32, distance: u32) ->u32 {
    let mut ways = 0;
    for hold_duration in 0..=time {
        let speed = hold_duration;
        let remaining_time = time - hold_duration;
        let distance_covered = speed * remaining_time;

        if distance_covered > distance {
            ways += 1;
        }
    }
    ways
}