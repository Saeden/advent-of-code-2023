use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    day3_1();
}

fn day3_1() {
    let file = File::open("test.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut output: usize = 0;

    for line in reader.lines() {
    }
    println!("\nOutput 1: {}", output)
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Number {
    start: Point,
    end: Point,
    value: usize,
}


#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}
