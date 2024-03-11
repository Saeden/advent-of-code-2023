use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn line_iterator(_path: &str) -> std::io::Lines<BufReader<File>>{
    let file = File::open("test.txt").expect("Failed to open file");
    let reader: BufReader<File> = BufReader::new(file);

    return reader.lines()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
