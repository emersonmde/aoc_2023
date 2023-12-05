use common::read_input;
use itertools::Itertools;
use regex::Regex;

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let input = match read_input("src/bin/day2/input.txt".to_string()) {
        Ok(input) => input,
        Err(error) => panic!("Error: {:?}", error),
    };
    let sum: u32 = input
        .iter()
        .flat_map(|game| {
            let (game_number, blocks_used) = find_rounds(game.to_string());
            parse_game(&blocks_used)
                .into_iter()
                .map(move |round| (game_number, round))
        })
        .group_by(|(game_number, _)| *game_number)
        .into_iter()
        .filter_map(|(game_number, group)| {
            let is_all = group.into_iter().all(|(_, round)| {
                round.red <= MAX_RED_CUBES
                    && round.green <= MAX_GREEN_CUBES
                    && round.blue <= MAX_BLUE_CUBES
            });
            if is_all {
                Some(game_number)
            } else {
                None
            }
        })
        .sum();
    println!("Sum of game IDs: {}", sum);

    let sum_of_min_blocks: u32 = input
        .iter()
        .flat_map(|game| {
            let (game_number, blocks_used) = find_rounds(game.to_string());
            parse_game(&blocks_used)
                .into_iter()
                .map(move |round| (game_number, round))
        })
        .group_by(|(game_number, _)| *game_number)
        .into_iter()
        .map(|(_, group)| {
            group.into_iter().fold(
                Round {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |mut acc, (_, round)| {
                    if round.red > acc.red {
                        acc.red = round.red;
                    }
                    if round.green > acc.green {
                        acc.green = round.green;
                    }
                    if round.blue > acc.blue {
                        acc.blue = round.blue;
                    }
                    acc
                },
            )
        })
        .map(|blocks| blocks.red * blocks.green * blocks.blue)
        .sum();
    println!("Sum of min blocks {}", sum_of_min_blocks)
}

fn find_rounds(game_string: String) -> (u32, String) {
    let game_regex = Regex::new(r"Game (\d+): (.*)").unwrap();
    let (game_number, blocks_used) = game_regex
        .captures(&game_string)
        .map(|caps| {
            (
                caps.get(1).map_or("", |m| m.as_str()),
                caps.get(2).map_or("", |m| m.as_str()),
            )
        })
        .expect("Game does not fit expected format: {}");

    let game_number = game_number.parse::<u32>().unwrap();
    (game_number, blocks_used.to_string())
}

fn parse_game(game: &String) -> Vec<Round> {
    let rounds = game.split(";").map(|s| s.trim()).collect::<Vec<&str>>();
    let mut blocks = Vec::new();
    for round in rounds {
        let round_regex = Regex::new(r"(\d+) (\w+)").unwrap();
        let blocks_used: Round = round_regex
            .captures_iter(&round)
            .map(|caps| {
                (
                    caps.get(2).map_or("", |m| m.as_str()),
                    caps.get(1).map_or("", |m| m.as_str()),
                )
            })
            .fold(
                Round {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |mut acc, (color, value)| {
                    let value = value.parse::<u32>().unwrap();
                    match color {
                        "red" => acc.red += value,
                        "green" => acc.green += value,
                        "blue" => acc.blue += value,
                        _ => println!("Unknown color: {}", color),
                    }
                    acc
                },
            );
        blocks.push(blocks_used);
    }
    blocks
}
