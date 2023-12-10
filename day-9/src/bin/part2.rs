use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    if let Ok(file) = File::open("./input.txt") {
        let reader = io::BufReader::new(file);
        let mut result = 0;
        for line_result in reader.lines() {
            if let Ok(line) = line_result {
                let vec = parse_line(&line);
                result += extrapolate_previous_value(vec);
            }
        }
        println!("{}", result);
    }
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace().map(|number| number.parse::<i32>().unwrap()).collect()
}

fn calculate_differences(sequence: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sequences: Vec<Vec<i32>> = vec![sequence];

    loop {
        let last_sequence = sequences.last().unwrap().clone();
        let mut new_sequence: Vec<i32> = vec![0; last_sequence.len() - 1];

        for i in 1..last_sequence.len() {
            new_sequence[i - 1] = last_sequence[i] - last_sequence[i - 1];
        }

        if new_sequence.iter().all(|&x| x == 0) {
            break;
        }

        sequences.push(new_sequence);
    }
    sequences.reverse();
    sequences
}

fn extrapolate_previous_value(sequence: Vec<i32>) -> i32 {
    let sequences = calculate_differences(sequence);
    let mut value = sequences[0][0];
    for i in 1..sequences.len() {
            value = sequences[i].first().unwrap() - value;
    }
    value
}