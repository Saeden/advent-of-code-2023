use std::collections::HashMap;

use utils::line_iterator;

fn main() {
    day3_1();
}

fn day3_1() {
    let mut output: usize = 0;
    let mut map: HashMap<Point, char> = HashMap::new();

    let mut number: usize = 0;
    let mut numbers: Vec<Number> = Vec::new();

    for (y, line) in line_iterator("test.txt").enumerate() {
        let line = line.expect("Failed to read line");
        println!("({}): line {}", line, y);
        for (x, character) in line.chars().enumerate() {
            if character.is_digit(10) {
                number = number * 10 + character.to_digit(10).unwrap() as usize;
            } else if !character.is_digit(10) && number != 0 {
                let end = Point { x: x - 1, y };
                let number_len = number.to_string().len();
                let start = Point {
                    x: x - number_len,
                    y,
                };
                numbers.push(Number {
                    start,
                    end,
                    value: number,
                });
                number = 0;
            }
            map.insert(Point { x, y }, character);
        }
    }

    println!("");

    for number in numbers {
        println!("{}", number.value.to_string());
        if has_special_neighbour(&number, &map) {
            output += number.value;
        }
    }

    println!("\nOutput 1: {}", output)
}

fn has_special_neighbour(number: &Number, map: &HashMap<Point, char>) -> bool {
    let mut has = false;
    let start = number.start.x;
    let end = number.end.x;
    let y = number.start.y;
    for i in start..end + 1 {
        if i == start && i != 0 {
            has = map.get(Point { x: i, y }).unwrap()
        }
    }
    true
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
