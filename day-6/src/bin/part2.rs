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
        let time = get_numbers(&time_string);
        let distance = get_numbers(&distance_string);
        result *= calculate_bounds(time, distance);

    }
    println!("{}", result);
}

fn get_numbers(stringed_numbers: &str) -> u64 {
    let mut result = stringed_numbers.split(":").nth(1).unwrap().to_owned();
    let final_number = result.chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<u64>().unwrap();
    final_number
}

fn calculate_bounds(time: u64, distance: u64) ->u64 {
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