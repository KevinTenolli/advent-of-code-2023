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
            dbg!(vertical);
            vertical_total += vertical;
        } else {
            horizontal_total += horizontal;
            dbg!(horizontal);
        }
    }
    let total = vertical_total + 100 * horizontal_total;
    dbg!(total);
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
            if *line == next_line_content {
                let mut left_index = line_index as i32 - 1;
                let mut right_index = line_index + 2;
                let mut is_equal = true;
                while left_index >= 0 && right_index < lines.len() {
                    if lines[left_index as usize] == lines[right_index] {
                        left_index -= 1;
                        right_index += 1;
                    } else {
                        is_equal = false;
                        break;
                    }
                }
                if is_equal {
                    return line_index + 1;
                }
            }
        }
    }
    0
}
