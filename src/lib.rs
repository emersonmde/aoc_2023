use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

// Read input file into a vector of strings
pub fn read_input(file_path: String) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let input = reader.lines().filter_map(|line| line.ok()).collect();
    Ok(input)
}
