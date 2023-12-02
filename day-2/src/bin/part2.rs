use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut result = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ln) = line {
                let parts: Vec<&str> = ln.split(":").collect();
                result += get_cube_power(parts[1]);
            }
        }
    }
    println!("{}",result);
}
fn get_cube_power(data: &str) -> u32 {
    let mut rgb_cubes_for_round = [0; 3];
    for round in data.split(";") {
        for cube in round.split(",") {
            let index = match cube {
                c if c.contains("red") => 0,
                c if c.contains("green") => 1,
                _ => 2,
            };
            let no_of_cubes = get_number(cube);
            if no_of_cubes > rgb_cubes_for_round[index] {
                rgb_cubes_for_round[index] = no_of_cubes
            }
        }
    }
    rgb_cubes_for_round.iter().fold(1, |acc, &x| acc * x)
}

fn get_number(st: &str) -> u32 {
    let result:Option<u32> = st
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .ok();
    match result {
        Some(res) => res,
        None => 0
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}