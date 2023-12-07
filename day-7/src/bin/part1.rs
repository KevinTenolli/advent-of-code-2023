use std::collections::{HashMap};
use std::fs::File;
use std::io;
use std::io::BufRead;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HIGH,       // 23456
    ONEPAIR,    // A23A4
    TWOPAIR,    // 23432
    THREEKIND,  // TTT98
    FULLHOUSE,  // 23332
    FOURKIND,   // AA8AA
    FIVEKIND,   // AAAAA
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CardLabel {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
impl CardLabel {
    fn from_letter(letter: char) ->Option<CardLabel> {
        match letter {
            '3' => Some(CardLabel::Three),
            '4' => Some(CardLabel::Four),
            '2' => Some(CardLabel::Two),
            '5' => Some(CardLabel::Five),
            '6' => Some(CardLabel::Six),
            '7' => Some(CardLabel::Seven),
            '8' => Some(CardLabel::Eight),
            '9' => Some(CardLabel::Nine),
            'T' => Some(CardLabel::Ten),
            'J' => Some(CardLabel::Jack),
            'Q' => Some(CardLabel::Queen),
            'K' => Some(CardLabel::King),
            'A' => Some(CardLabel::Ace),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct CardHand {
    hand: String,
    hand_type: HandType,
    rank: u32
}
impl  CardHand {
    fn new(hand: &str, rank: u32) -> CardHand {
        let hand_type = CardHand::get_hand_type(hand);
        CardHand {
            hand: hand.to_string(),
            hand_type,
            rank,
        }
    }

    fn get_hand_type(hand: &str) -> HandType {
        let mut label_counts: HashMap<char, u32> = HashMap::new();
        for label in hand.chars() {
            let count = label_counts.entry(label).or_insert(0);
            *count += 1;
        }
        let max_count = *label_counts.values().max().unwrap_or(&0);
        let unique_labels = label_counts.keys().count();
        match (max_count, unique_labels) {
            (5, 1) => HandType::FIVEKIND,
            (4, 2) => HandType::FOURKIND,
            (3, 2) => HandType::FULLHOUSE,
            (3, 3) => HandType::THREEKIND,
            (2, 3) => HandType::TWOPAIR,
            (2, 4) => HandType::ONEPAIR,
            _ => HandType::HIGH,
        }
    }
}

fn main() {
    if let Ok(file) = File::open("./input.txt") {
        let mut card_list:Vec<CardHand> = Vec::new();
        let reader = io::BufReader::new(file);
        for line_result in reader.lines() {
            if let Ok(line) = line_result {
                let data = parse_line(&line);
                let card_hand = CardHand::new(&data.0, data.1);
                add_card_to_list(&mut card_list, card_hand);
            }
        }
        let mut result = 0;
        for (idx, card) in card_list.iter().enumerate() {
            result+= (idx as u32+1) * card.rank;
        }
        println!("{}",result);
    }
}
fn add_card_to_list(list: &mut Vec<CardHand>, card_hand: CardHand) {
    let mut insert_position = None;
    for (i, existing_hand) in list.iter().enumerate() {
        if existing_hand.hand_type > card_hand.hand_type {
            insert_position = Some(i);
            break;
        } else if existing_hand.hand_type == card_hand.hand_type {
            match is_new_hand_bigger(&card_hand.hand, &existing_hand.hand){
                false => insert_position = Some(i),
                true => insert_position = Some(i+1),
            }
            break;
        }
    }

    if let Some(position) = insert_position {
        list.insert(position, card_hand);
    } else {
        list.extend(std::iter::once(card_hand));
    }
}
fn parse_line(line: &str)  -> (String, u32) {
    let mut data = line.split_whitespace();
    let card_hand = data.next().unwrap().to_owned();
    let value = data.next().unwrap().parse::<u32>().unwrap();
    (card_hand, value)
}

fn is_new_hand_bigger(new_hand: &str, old_hand: &str) ->bool {
    for (new_card, old_card) in new_hand.chars().zip(old_hand.chars()) {
        if let (Some(new_label), Some(old_label)) = (CardLabel::from_letter(new_card), CardLabel::from_letter(old_card)) {
            if new_label > old_label {
                return true;
            } else if old_label > new_label {
                return false;
            }
        }
    }
    return false;
}