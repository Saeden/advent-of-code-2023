use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::num;

fn main() {
    day2_1();
}

fn day2_1() {
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut output: usize = 0;

    for line in reader.lines() {
        let complete = check_game(&line);

        if complete {
            let id = get_id(&line);
            println!("id: {}", id);
            output += id as usize;
        }
    }
    println!("\nOutput: {}", output)
}

fn check_game(line_result: &Result<String, Error>) -> bool{
    let line = line_result.as_ref().expect("Error getting line");
    let tokens: Vec<&str> = line.split(|c: char| c.is_whitespace() || c.is_ascii_punctuation()).collect();
    for i in (4..tokens.len()).step_by(3) {
        let colour = tokens[i];
        if let Ok(balls) = tokens[i - 1].parse::<u32>(){
            // println!("{:?} {}", balls, colour);
            if colour == "red" &&  balls > 12 {return false}
            else if colour == "green" && balls > 13 {return false}
            else if colour == "blue" && balls > 14 {return false}
            else if !["red", "green", "blue"].contains(&colour) {
                println!("Bad tokens. Balls: {} | Colour: {}", tokens[i-1], tokens[i])
            }
        } else {
            panic!("Bad tokens. Balls: {} | Colour: {}", tokens[i-1], tokens[i])
        }
    }
    return true;
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
            break;
        }
    } 
        
    for i in (0..=digits.len()-1) {
        let power = digits.len() - i - 1; 
        id += digits[i] * 10_u8.pow(power as u32);
        // println!("Digit: {}, Power: {}, ID: {}", digits[i], power, id);
    }
    return id;
}
