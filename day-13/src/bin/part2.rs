use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let blocks = read_blocks();
    let mut vertical_total = 0;
    let mut horizontal_total = 0;
    for block in blocks {
        let horizontal = find_horizontal_mirror(&block);
        if horizontal == 0 {
            let vertical = find_vertical_mirror(&block);
            vertical_total += vertical;
        } else {
            horizontal_total += horizontal;
        }
    }
    let total = vertical_total + 100 * horizontal_total;
    println!("{}",total);
}

fn read_blocks() -> Vec<String> {
    let mut blocks = Vec::new();
    let mut block = String::new();
    if let Ok(file) = File::open("./input.txt") {
        let reader = io::BufReader::new(file);
        for line_result in reader.lines() {
            if let Ok(line) = line_result {
                if line.trim().is_empty() {
                    blocks.push(block.clone());
                    block.clear();
                }
                else {
                    block.push_str(&line);
                    block.push('\n');
                }
            }
        }
    }
    if !block.is_empty() {
        blocks.push(block);
    }
    blocks
}
fn find_vertical_mirror(block: &str) -> usize {
    let lines: Vec<&str> = block.lines().collect();
    let transposed = transpose_block(&lines);
    find_horizontal_mirror(&transposed)
}

fn transpose_block(block: &Vec<&str>) -> String {
    let num_rows = block.len();
    let num_cols = block[0].len();

    let transposed_block: Vec<String> = (0..num_cols)
        .map(|col| (0..num_rows).map(|row| block[row].chars().nth(col).unwrap()).collect())
        .collect();

    transposed_block.join("\n")

}

fn find_horizontal_mirror(block: &str) -> usize {
    let lines: Vec<&str> = block.lines().collect();
    for (line_index, line) in lines.iter().enumerate() {
        let next_line = if line_index < lines.len() - 1 { Some(lines[line_index + 1]) } else { None };
        if let Some(next_line_content) = next_line {
             if *line == next_line_content || is_off_by_one(&line, &next_line_content) {
                if let Some(_result) = check_for_mirror(&lines, line_index) {
                    return line_index + 1;
                }
            }
        }
    }
    0
}

fn check_for_mirror(lines: &Vec<&str>, line_index: usize) -> Option<usize> {
    let mut left_index = line_index as i32 - 1;
    let mut right_index = line_index + 2;
    let mut is_equal = true;
    while left_index >= 0 && right_index < lines.len() {
        if lines[left_index as usize] == lines[right_index] || is_off_by_one(&lines[left_index as usize], &lines[right_index]) {
            left_index -= 1;
            right_index += 1;
        } else {
            is_equal = false;
            break;
        }
    }
    match is_equal {
        true => Some(line_index + 1),
        false => None,
    }
}

fn is_off_by_one(str1: &str, str2: &str) -> bool {
    if str1.len() != str2.len() {
        return  false;
    }
    let mut differences = 0;
    for (char1, char2) in str1.chars().zip(str2.chars()) {
        if char1 != char2 {
            differences += 1;
        }
    }
    match differences {
        1 => true,
        _ => false
    }
}
