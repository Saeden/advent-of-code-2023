use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    //day1_1();
    day1_2();
}

fn day1_2() {
    let file = File::open("test2.txt").expect("Failed to open file");
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
                println!("sub string before check: {}", sub_string);
                if check_str(&mut sub_string){
                    let digit = convert_to_digit(&mut sub_string);
                    digits.push(digit);
                    sub_string = "".to_string();
                }
                println!("sub string after check: {}", sub_string);
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
            println!("str check succes: str({:?}) == num({:?})", sub_string, number);
            return true;
        } 
        println!("number == {:?}", number);
        let num_sub_str = &number.to_string()[..sub_string.len()];
        println!("num sub str == to len sub_str: {:?} == {:?}", num_sub_str, sub_string);
        if  *num_sub_str == *sub_string{
            println!("str is sub str");
            return false;
        }
    }
    println!("str is not a sub str");
    *sub_string = "".to_string();
    return false
}

fn convert_to_digit(sub_string: &String) -> usize{
    match sub_string.as_str() {
       "one"   => return 1,
       "two"   => return 2,
       "three" => return 3,
       "four"  => return 4,
       "five"  => return 5,
       "six"   => return 6,
       "seven" => return 7,
       "eight" => return 8,
       "nine"  => return 9,
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
