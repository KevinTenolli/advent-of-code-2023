use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("./input.txt").expect("Error opening file");
    let reader = io::BufReader::new(file);
    let mut map_of_tickets: HashMap<u32, u32> = HashMap::new();
    let mut total_tickets:u32 = 0;
    let mut idx:u32 = 1;

    for (_, line_result) in reader.lines().enumerate() {
        if let Ok(line) = line_result {
            let mut card_iterator = line.split(":").nth(1).unwrap().split("|").into_iter();
            //get how many cards in the future will get a bonus
            let range_of_bonus = get_line_result(card_iterator.next().unwrap(), card_iterator.next().unwrap());
            //get how many duplicates of the current card there are
            let mut current_number_of_cards_at_index = *map_of_tickets.get(&idx).unwrap_or(&0);
            //add the original card
            current_number_of_cards_at_index += 1;
            //for each card in the future
            for i in 1..range_of_bonus + 1  {
                //if they have duplicates, add more duplicates
                if let Some(value) = map_of_tickets.get_mut(&(idx + i)) {
                    *value += current_number_of_cards_at_index;
                } else {
                    //add first set of duplicates
                    map_of_tickets.insert(idx + i,  current_number_of_cards_at_index);
                }
            }
            //remove duplicates for current card
            map_of_tickets.remove(&idx);
            // add to total tickets
            total_tickets += current_number_of_cards_at_index;
            idx += 1;
        }
    }
    println!("{}", total_tickets);
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
            points += 1
        }
    });
    points
}
fn parse_number(stringed_number: &str) -> u32 {
    stringed_number.parse::<u32>().unwrap_or(0)
}