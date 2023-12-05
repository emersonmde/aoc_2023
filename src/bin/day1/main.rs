use std::collections::HashMap;

use regex::Regex;

use common::read_input;

fn main() {
    let input = match read_input("src/bin/day1/input.txt".to_string()) {
        Ok(input) => input,
        Err(error) => panic!("Error: {:?}", error),
    };

    let mut sum = 0;
    for line in &input {
        let (first, last) = find_digits2(line);
        // if either first or last are none
        if first.is_none() || last.is_none() {
            println!("No digits found in line: {}", line);
            continue;
        }
        sum += (first.unwrap() * 10) + last.unwrap();
    }
    println!("Sum: {}", sum);
}

fn find_digits(input: &String) -> (Option<u32>, Option<u32>) {
    let digits: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();

    if digits.is_empty() {
        return (None, None);
    }

    (digits.first().cloned(), digits.last().cloned())
}

fn find_digits2(input: &String) -> (Option<u32>, Option<u32>) {
    let digits_numbers: Vec<String> = (0..=9).map(|d| d.to_string()).collect();
    let digit_names = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    // Construct a map of digit names to their corresponding number
    let digit_map: HashMap<_, _> = digit_names
        .iter()
        .enumerate()
        .map(|(index, &word)| (word, index as u32 + 1))
        .collect();

    // Combine digit names and numbers into a single vector
    let combined_digits: Vec<String> = digit_names
        .iter()
        .map(|&name| name.to_string())
        .chain(digits_numbers.iter().cloned())
        .collect();

    // Find the first match
    let forward_pattern = combined_digits.join("|");
    let forward_regex = Regex::new(&forward_pattern).unwrap();
    let forward_match = forward_regex.find(input);
    let first_match = forward_match.map(|m| m.as_str());

    // Find the last match
    // Rust doesn't support forward looking matches so we reverse the input and the pattern
    let reverse_pattern = combined_digits
        .iter()
        .map(|s| s.chars().rev().collect::<String>())
        .rev()
        .collect::<Vec<String>>()
        .join("|");
    let reverse_regex = Regex::new(&reverse_pattern).unwrap();
    let reverse_input = input.chars().rev().collect::<String>();
    let reverse_match = reverse_regex.find(&reverse_input);
    let last_match = reverse_match.map(|m| m.as_str());

    if first_match.is_none() && last_match.is_none() {
        println!("No digits found in line: {}", input);
        return (None, None);
    }

    let (first, last) = match (first_match, last_match) {
        (Some(first), Some(last)) => (first, last),
        (Some(first), None) => (first, first),
        (None, Some(last)) => (last, last),
        _ => return (None, None),
    };

    let last = last.chars().rev().collect::<String>();

    (
        first
            .parse::<u32>()
            .ok()
            .or_else(|| digit_map.get(first).cloned()),
        last.parse::<u32>()
            .ok()
            .or_else(|| digit_map.get(last.as_str()).cloned()),
    )
}
