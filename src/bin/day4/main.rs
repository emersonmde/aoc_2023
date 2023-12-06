use std::collections::{BTreeSet, HashMap};
use itertools::Itertools;
use regex::Regex;
use common::read_input;

fn main() {
    let input = match read_input("src/bin/day4/input.txt".to_string()) {
        Ok(input) => input,
        Err(error) => panic!("Error: {:?}", error),
    };

    // Create list of card_ids and the number of winning numbers for each card
    let cards = input.iter()
        .map(|card| parse_card(card.to_string()))
        .map(|card| (card.card_id, card.winning_numbers.intersection(&card.your_numbers).count()))
        .collect::<Vec<_>>();

    let total_points: u32 = cards.iter()
        .filter_map(|(_, count)| if *count > 0 { Some(*count as u32) } else { None })
        .map(|count| 2_u32.pow(count - 1))
        .sum();
    println!("Total points: {}", total_points);

    // Create lookup table for the number of winning numbers for each card
    let card_winning_num_map: HashMap<u32, u32> = cards.iter()
        .fold(HashMap::new(), |mut acc, (card_id, count)| {
            acc.insert(*card_id, *count as u32);
            acc
        });

    // Pre-populate work stack
    let mut card_stack = cards.iter()
        .map(|(card_id, _)| *card_id)
        .sorted()
        .rev()
        .collect::<Vec<_>>();


    let mut count = 0;
    while !card_stack.is_empty() {
        let card_id = card_stack.pop().unwrap();
        let num_winning_numbers = *card_winning_num_map.get(&card_id).unwrap();
        let duplicated_cards = (card_id + 1..=card_id + num_winning_numbers).rev().collect::<Vec<_>>();
        card_stack.extend(duplicated_cards.into_iter());
        count += 1;
    }
    println!("Count: {}", count);
}


#[derive(Debug)]
struct Card {
    card_id: u32,
    winning_numbers: BTreeSet<u32>,
    your_numbers: BTreeSet<u32>,

}

fn parse_card(card: String) -> Card {
    let re = Regex::new(r"Card\s+(\d+): ([\d ]+) \| ([\d ]+)").unwrap();
    // return the card_id, winning numbers, and your numbers
    let captures = re.captures(&card).unwrap();
    let card_id = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let winning_numbers = captures.get(2).unwrap().as_str().split(" ")
        .filter(|x| x != &"")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<BTreeSet<u32>>();
    let your_numbers = captures.get(3).unwrap().as_str().split(" ")
        .filter(|x| x != &"")
        .map(|x| x.parse::<u32>().expect("Error parsing your numbers"))
        .collect::<BTreeSet<u32>>();
    Card {
        card_id,
        winning_numbers,
        your_numbers,
    }
}