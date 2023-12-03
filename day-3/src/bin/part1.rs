use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
#[derive(Debug)]
struct Block {
    prev: Option<String>,
    curr: Option<String>,
    next: Option<String>
}
impl Block {
    fn new() -> Block {
        Block {
            prev: None,
            curr: None,
            next: None
        }
    }
    fn populate(&mut self, line: String) {
        if self.prev.is_none() {
            self.prev = Some(line);
        } else if self.curr.is_none() {
            self.curr = Some(line);
        } else if self.next.is_none() {
            self.next = Some(line);
        }
    }
    fn shift_left(&mut self) {
        if self.next.is_some() {
            self.prev = self.curr.take();
            self.curr = self.next.take();
            self.next = None;
        }
    }
}
fn main() {
    let mut result = 0;
    let mut block = Block::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                block.shift_left();
                block.populate(ip.clone());
                dbg!(&block);
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
