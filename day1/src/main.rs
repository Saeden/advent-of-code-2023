use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut output: Vec<usize> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut digits: Vec<usize> = Vec::new();
        for character in line.chars() {
            if character.is_digit(10){
                digits.push((character as usize) - ('0' as usize));
            }
        }
        let mut digit = digits[0] * 10;
        digit = digit + digits[digits.len()-1];
        output.push(digit);
    }

    let num: usize = output.iter().sum();
    println!("{}", num);
}

