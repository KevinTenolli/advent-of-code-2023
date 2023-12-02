use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut result = 0;
    let rgb_cubes:[u32;3] = [12,13,14];
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ln) = line {
                let parts: Vec<&str> = ln.split(":").collect();
                let is_possible = is_game_possible(parts[1], &rgb_cubes);
                if is_possible {
                    result += get_number(parts[0]);
                }
            }
        }
    }
    println!("{}",result);
}
fn is_game_possible(data: &str, rgb_cubes: &[u32; 3]) -> bool {
    for round in data.split(";") {
        let mut rgb_cubes_for_round = [0; 3];

        for cube in round.split(",") {
            let index = match cube {
                c if c.contains("red") => 0,
                c if c.contains("green") => 1,
                _ => 2,
            };

            rgb_cubes_for_round[index] = get_number(cube);
        }

        if rgb_cubes_for_round[0] > rgb_cubes[0]
            || rgb_cubes_for_round[1] > rgb_cubes[1]
            || rgb_cubes_for_round[2] > rgb_cubes[2]
        {
            return false;
        }
    }
    true
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