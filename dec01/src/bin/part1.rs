use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());

    let sum = reader
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let mut digits = line.chars().filter(|c| c.is_numeric());

            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            format!("{first}{last}").parse::<i32>().unwrap()
        })
        .sum::<i32>();

    println!("{sum}");
}
