use std::cmp;
use common::read_input;

#[derive(Debug)]
struct Number {
    position: (u32, u32),
    length: u32,
}

fn main() {
    let input: Vec<Vec<char>> = match read_input("src/bin/day3/input.txt".to_string()) {
        Ok(input) => input.iter()
            .map(|line| line.chars().collect())
            .collect(),
        Err(error) => panic!("Error: {:?}", error),
    };

    let mut numbers: Vec<Number> = Vec::new();
    for (line_num, line) in input.iter().enumerate() {
        let numbers_in_line = find_numbers_in_line(line, line_num as u32);
        numbers.extend(numbers_in_line);
    }

    let mut i = 0;
    for number in &numbers {
        if is_part_number(&number, &input.clone()) {
            let (x, y) = number.position;
            let number_string: String = input[y as usize].to_owned().into_iter().collect();
            let number_string = &number_string[x as usize..(x + number.length) as usize].to_string();
            let number_value = number_string.parse::<u32>().unwrap();

            i += number_value;
            println!("Part number: {:?}", &number_value);
        }
    }
    println!("Part numbers: {}", i);
}

fn find_numbers_in_line(input: &Vec<char>, line_num: u32) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();

    let mut current_position: Option<(u32, u32)> = None;
    let mut current_length: Option<u32> = None;

    for (index, &character) in input.iter().enumerate() {
        if character.is_digit(10) {
            if current_position.is_none() {
                current_position = Some((index as u32, line_num));
                current_length = Some(1);
            } else {
                current_length = Some(current_length.unwrap() + 1);
            }
        } else {
            if current_position.is_some() {
                numbers.push(Number {
                    position: current_position.unwrap(),
                    length: current_length.unwrap(),
                });
                current_length = None;
                current_position = None;
            }
        }
    }

    numbers
}

fn is_part_number(number: &Number, schematic: &Vec<Vec<char>>) -> bool {
    let (x, y) = number.position;
    let length = number.length;
    let max_x = schematic[0].len() as u32;
    let max_y = schematic.len() as u32;

    let start = cmp::max(x, 1) - 1;
    let end = cmp::min(x + length, max_x - 1);
    let yrange = [cmp::max(y, 1) - 1, y, cmp::min(y + 1, max_y - 1)];

    for y in yrange.iter() {
        for x in start..end {
            let character = schematic[*y as usize].get(x as usize).expect("Character not found");
            if !character.is_alphanumeric() && character != &'.' {
                println!("Character {:?} at position ({}, {}) is not alphanumeric", character, x, y);
                return true;
            }
        }
    }

    false
}