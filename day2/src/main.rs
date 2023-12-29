use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() {
    day2_1();
}

fn day2_1() {
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut output: Vec<usize> = Vec::new();

    for line in reader.lines() {
        let id = get_id(&line);
        println!("id: {}", id);
    }
}

fn get_id(line_result: &Result<String, Error>) -> u8{
    let mut digits: Vec<u8>  = Vec::new();
    let mut found   = false;
    let mut id      = 0;

    let line = line_result.as_ref().expect("Error getting line");
    for char in line.chars() {
        if found == true && char != ':' {
            digits.push((char as u8) - ('0' as u8))
        } else if char == ' ' {
            found = true;
        } else if char == ':'{

        }
    } 
    for digit in &digits {
        id += digit * 10_u8.pow(digits.len() as u32 - 1);
    }
    return id;
}
