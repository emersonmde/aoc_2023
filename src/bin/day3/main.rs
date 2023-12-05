use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::rc::Rc;
use common::read_input;

#[derive(Debug, Clone)]
struct Number {
    value: u32,
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

    let mut part_number_sum = 0;
    let gears: Rc<RefCell<HashMap<(u32, u32), Vec<Number>>>> = Rc::new(RefCell::new(HashMap::new()));
    for number in &numbers {
        if is_part_number((*number).clone(), &input.clone(), gears.clone()) {
            let (x, y) = number.position;
            let number_string: String = input[y as usize].to_owned().into_iter().collect();
            let number_string = &number_string[x as usize..(x + number.length) as usize].to_string();
            let number_value = number_string.parse::<u32>().unwrap();

            part_number_sum += number_value;
            println!("Part number: {:?}", &number_value);
        }
    }
    println!("Part numbers: {}", part_number_sum);

    let sum_of_gear_ratios: u64 = gears.borrow().iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().map(|n| n.value as u64).product::<u64>())
        .sum();
    println!("Sum of gear ratios: {}", sum_of_gear_ratios);
}

fn find_numbers_in_line(input: &Vec<char>, line_num: u32) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();

    let mut current_number: Option<String> = None;
    let mut current_position: Option<(u32, u32)> = None;
    let mut current_length: Option<u32> = None;

    for (index, &character) in input.iter().enumerate() {
        if character.is_digit(10) {
            if current_position.is_none() {
                current_position = Some((index as u32, line_num));
                current_length = Some(1);
                current_number = Some(character.to_string());
            } else {
                current_length = Some(current_length.unwrap() + 1);
                current_number = Some(format!("{}{}", current_number.unwrap(), character));
            }
        } else {
            if current_position.is_some() {
                numbers.push(Number {
                    value: current_number.unwrap().parse::<u32>().unwrap(),
                    position: current_position.unwrap(),
                    length: current_length.unwrap(),
                });
                current_number = None;
                current_length = None;
                current_position = None;
            }
        }
    }

    if let Some(position) = current_position {
        numbers.push(Number {
            value: current_number.unwrap().parse::<u32>().unwrap(),
            position: position,
            length: current_length.unwrap(),
        });
    }

    numbers
}

fn is_part_number(number: Number, schematic: &Vec<Vec<char>>, mut gears: Rc<RefCell<HashMap<(u32, u32), Vec<Number>>>>) -> bool {
    let (x, y) = number.position;
    let length = number.length;
    let max_x = schematic[0].len() as u32;
    let max_y = schematic.len() as u32;

    let start_x = if x > 0 { x - 1 } else { 0 };
    let end_x = cmp::min(x + length, max_x - 1);
    let start_y = if y > 0 { y - 1 } else { 0 };
    let end_y = cmp::min(y + 1, max_y - 1);

    for y in start_y..=end_y {
        for x in start_x..=end_x {
            let character = schematic[y as usize].get(x as usize).expect("Character not found");
            if character == &'*' {
                // Add Number to (x, y) in gears
                let mut gears = gears.borrow_mut();
                let mut numbers = gears.entry((x, y)).or_insert(Vec::new());
                numbers.push(number.clone());
            }
            if !character.is_numeric() && character != &'.' {
                println!("Character {:?} at position ({}, {}) is a part number", character, x, y);
                return true;
            }
        }
    }

    false
}