use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut totals = Vec::<u32>::new();
    for line in reader.lines() {
        let mut digits = Vec::<u32>::new();
        let l = line.unwrap();
        for c in l.chars() {
            if c.is_digit(10) {
                let d = c.to_digit(10).unwrap();
                digits.push(d)
            }
        }
        let first = digits[0];
        let last = digits[digits.len() -1];
        let num_string = format!("{}{}", first, last);
        let num = num_string.parse::<u32>().unwrap();
        totals.push(num);
    }
    let sum: u32 = totals.iter().sum();
    println!("{}", sum);
}
