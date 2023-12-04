use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn str_to_char(val: &str) -> Option<char> {
    if val.starts_with("one") || val.starts_with('1') {
        Some('1')
    } else if val.starts_with("two") || val.starts_with('2') {
        Some('2')
    } else if val.starts_with("three") || val.starts_with('3') {
        Some('3')
    } else if val.starts_with("four") || val.starts_with('4') {
        Some('4')
    } else if val.starts_with("five") || val.starts_with('5') {
        Some('5')
    } else if val.starts_with("six") || val.starts_with('6') {
        Some('6')
    } else if val.starts_with("seven") || val.starts_with('7') {
        Some('7')
    } else if val.starts_with("eight") || val.starts_with('8') {
        Some('8')
    } else if val.starts_with("nine") || val.starts_with('9') {
        Some('9')
    } else {
        None
    }
}

fn get_single_digit(val: &str) -> Option<char> {
    if val.is_empty() {
        return None;
    }
    match str_to_char(val) {
        Some(c) => Some(c),
        None => get_single_digit(&val[1..]),
    }
}

fn get_value(val: &str) -> i32 {
    let first = get_single_digit(val).unwrap();
    let mut sub = &val[1..];
    let mut last = first;
    while !sub.is_empty() {
        if let Some(c) = str_to_char(sub) {
            last = c;
        }
        sub = &sub[1..];
    }
    format!("{first}{last}").parse().unwrap()
}

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());

    let sum = reader
        .lines()
        .map(Result::unwrap)
        .map(|line| get_value(&line))
        .sum::<i32>();

    println!("{sum}");
}
