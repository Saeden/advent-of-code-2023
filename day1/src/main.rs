use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    day1_1();
    day1_2();
}

fn day1_2() {
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut output: Vec<usize> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut digits: Vec<usize> = Vec::new();
        let mut sub_string: String = String::new();

        for character in line.chars() {
            if character.is_digit(10){
                digits.push((character as usize) - ('0' as usize));
            } else if character.is_alphabetic() {
                sub_string = sub_string + &String::from(character);
                if check_str(&mut sub_string){
                    let found_digit = convert_to_digit(&mut sub_string);
                    digits.push(found_digit);
                }
            }
        }
        let mut digit = digits[0] * 10;
        digit = digit + digits[digits.len()-1];
        output.push(digit);
    }

    let num: usize = output.iter().sum();
    println!("{}", num);
}

fn check_str(sub_string: &mut String) -> bool{
    let numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for number in numbers {
        if sub_string.len() > number.len() {
            continue;
        } else if *sub_string == number {
            return true;
        } else if  number.to_string()[..sub_string.len()] == *sub_string{
            return false;
        }
    }
    *sub_string = sub_string[1..].to_string();
    return false
}

fn convert_to_digit(sub_string: &mut String) -> usize{
    match sub_string.as_str() {
       "one"   => {*sub_string = String::from("e");return 1},
       "two"   => {*sub_string = String::from("o");return 2},
       "three" => {*sub_string = String::from("e");return 3},
       "four"  => {*sub_string = String::from(""); return 4},
       "five"  => {*sub_string = String::from("e");return 5},
       "six"   => {*sub_string = String::from("");return 6},
       "seven" => {*sub_string = String::from("n");return 7},
       "eight" => {*sub_string = String::from("t");return 8},
       "nine"  => {*sub_string = String::from("e");return 9},
        _      => panic!("Something is triggering as a digit: {}", sub_string),
    }
}

fn day1_1() {
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
