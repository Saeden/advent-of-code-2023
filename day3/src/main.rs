use std::fs::File;
use std::io::{BufReader, BufRead};
use utils::line_iterator;

fn main() {
    day3_1();
}

fn day3_1() {
    let mut output: usize = 0;

    for line in line_iterator("test.txt") {
        
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
