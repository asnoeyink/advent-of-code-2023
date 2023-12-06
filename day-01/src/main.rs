use std::cmp::min;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let num_words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut totals = Vec::<u32>::new();
    for line in reader.lines() {
        let mut digits = Vec::<u32>::new();
        let mut words = HashMap::<u32, String>::new();
        let l = line.unwrap();
        let mut i = 0;
        let mut window_start = 0;
        for c in l.chars() {
            if c.is_digit(10) {
                let d = c.to_digit(10).unwrap();
                digits.push(d);
            } else {
                window_start = i;
                let window_end = min(window_start + 5, l.len());
                let window = l.chars().skip(window_start).take(window_end).collect::<String>();
                let mut j = 1;
                for word in num_words {
                    if window.starts_with(word) {
                        digits.push(j);
                        break;
                    }
                    j += 1;
                }
            }

            i += 1;
        }
        let first = digits[0];
        let last = digits[digits.len() - 1];
        let num_string = format!("{}{}", first, last);
        let num = num_string.parse::<u32>().unwrap();
        totals.push(num);
    }
    let sum: u32 = totals.iter().sum();
    println!("{}", sum);
}
