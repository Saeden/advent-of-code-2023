use std::collections::HashMap;

use utils::line_iterator;

fn main() {
    //day3_1();
    day3_2();
}

fn day3_2() {
    let mut output: i32 = 0;
    let mut map: HashMap<Point, char> = HashMap::new();

    let mut number: i32 = 0;
    let mut numbers: Vec<Number> = Vec::new();
    for (y, line) in line_iterator("test.txt").enumerate() {
        let line = line.expect("Failed to read line");
        //println!("({}): line {}", line, y);
        for (x, character) in line.chars().enumerate() {
            if character.is_digit(10) && x == line.len() - 1 {
                number = number * 10 + character.to_digit(10).unwrap() as i32;
                let end = Point {
                    x: x as i32,
                    y: y as i32,
                };
                let number_len = number.to_string().len() as i32;
                let start = Point {
                    x: x as i32 - number_len,
                    y: y as i32,
                };
                numbers.push(Number {
                    start,
                    end,
                    value: number,
                });
                number = 0;
            } else if character.is_digit(10) {
                number = number * 10 + character.to_digit(10).unwrap() as i32;
            } else if !character.is_digit(10) && number != 0 {
                let end = Point {
                    x: x as i32 - 1,
                    y: y as i32,
                };
                let number_len = number.to_string().len() as i32;
                let start = Point {
                    x: x as i32 - number_len,
                    y: y as i32,
                };
                numbers.push(Number {
                    start,
                    end,
                    value: number,
                });
                number = 0;
            }
            map.insert(
                Point {
                    x: x as i32,
                    y: y as i32,
                },
                character,
            );
        }
    }

    for number in numbers {
        if has_special_neighbour(&number, &map) {
            println!("{}", number.value.to_string());
            output += number.value;
        }
    }
}

fn 

fn day3_1() {
    let mut output: i32 = 0;
    let mut map: HashMap<Point, char> = HashMap::new();

    let mut number: i32 = 0;
    let mut numbers: Vec<Number> = Vec::new();

    for (y, line) in line_iterator("input.txt").enumerate() {
        let line = line.expect("Failed to read line");
        //println!("({}): line {}", line, y);
        for (x, character) in line.chars().enumerate() {
            if character.is_digit(10) && x == line.len() - 1 {
                number = number * 10 + character.to_digit(10).unwrap() as i32;
                let end = Point {
                    x: x as i32,
                    y: y as i32,
                };
                let number_len = number.to_string().len() as i32;
                let start = Point {
                    x: x as i32 - number_len,
                    y: y as i32,
                };
                numbers.push(Number {
                    start,
                    end,
                    value: number,
                });
                number = 0;
            } else if character.is_digit(10) {
                number = number * 10 + character.to_digit(10).unwrap() as i32;
            } else if !character.is_digit(10) && number != 0 {
                let end = Point {
                    x: x as i32 - 1,
                    y: y as i32,
                };
                let number_len = number.to_string().len() as i32;
                let start = Point {
                    x: x as i32 - number_len,
                    y: y as i32,
                };
                numbers.push(Number {
                    start,
                    end,
                    value: number,
                });
                number = 0;
            }
            map.insert(
                Point {
                    x: x as i32,
                    y: y as i32,
                },
                character,
            );
        }
    }

    for number in numbers {
        if has_special_neighbour(&number, &map) {
            println!("{}", number.value.to_string());
            output += number.value;
        }
    }

    println!("\nOutput 1: {}", output)
}

fn has_special_neighbour(number: &Number, map: &HashMap<Point, char>) -> bool {
    let start = number.start.x;
    let end = number.end.x;
    let y = number.start.y;
    for i in start..=end {
        for neighbour_point in all_neighbour_points(i, y) {
            let neighbour_char_result = map.get(&neighbour_point);
            let neighbour_char = if let Some(inner) = neighbour_char_result {
                inner
            } else {
                &'0'
            };
            if is_special_char(neighbour_char) {
                return true;
            }
        }
    }
    false
}

fn all_neighbour_points(x: i32, y: i32) -> [Point; 8] {
    [
        Point { x: x - 1, y: y - 1 },
        Point { x: x - 1, y },
        Point { x: x - 1, y: y + 1 },
        Point { x, y: y - 1 },
        Point { x, y: y + 1 },
        Point { x: x + 1, y: y - 1 },
        Point { x: x + 1, y },
        Point { x: x + 1, y: y + 1 },
    ]
}

fn is_special_char(char_at_p: &char) -> bool {
    if ['+', '/', '=', '*', '@', '%', '-', '#', '&', '$'].contains(char_at_p) {
        return true;
    }
    false
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Number {
    start: Point,
    end: Point,
    value: i32,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
