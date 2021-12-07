use std::fs;
use std::ops::Add;

fn look_and_say(input: String) -> String {
    let mut current_char: char = '0';
    let mut count: u64 = 0;
    let mut collecting = false;
    let mut new_string = String::new();
    for c in input.chars() {
        if !collecting {
            collecting = true;
            count = 1;
            current_char = c;
        } else if c != current_char {
            new_string.push_str(&count.to_string());
            new_string.push(current_char);
            count = 1;
            current_char = c;
        } else {
            count += 1;
        }
    }
    new_string.push_str(&count.to_string());
    new_string.push(current_char);
    new_string
}

fn part1(input: &str) {
    let mut result = input.to_string();
    for _ in 0..40 {
        result = look_and_say(result);
    }
    println!("part1: {}", result.len());
}

fn part2(input: &str) {
    let mut result = input.to_string();
    for _ in 0..50 {
        result = look_and_say(result);
    }
    println!("part1: {}", result.len());
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
