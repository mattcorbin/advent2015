use std::fs;

fn vowel_check(input: &str) -> bool {
    let mut count = 0;
    let vowels = vec!["a", "e", "i", "o", "u"];
    for vowel in vowels.iter() {
        count += input.matches(vowel).count()
    }
    count >= 3
}

fn double_check(input: &str) -> bool {
    let mut last_char = input.chars().nth(0).unwrap();
    let mut current_char = input.chars().nth(1).unwrap();
    for c in input.chars().skip(2) {
        if last_char == current_char {
            return true;
        }
        last_char = current_char;
        current_char = c;
    }
    if last_char == current_char {
        return true;
    }
    false
}

fn no_bad_strings(input: &str) -> bool {
    let bad_strings = vec!["ab", "cd", "pq", "xy"];
    for test in bad_strings.iter() {
        if input.contains(test) {
            return false;
        }
    }
    true
}

fn non_overlapping_pairs(input: &str) -> bool {
    let bytes = input.as_bytes();
    for i in 1..bytes.len() {
        let first = bytes[i-1];
        let second = bytes[i];
        for j in 1..bytes.len() {
            if j == i - 1 || j == i || j == i + 1 {
                continue;
            }
            if bytes[j-1] == first && bytes[j] == second {
                return true;
            }
        }
    }
    false
}

fn repeat_with_interrupt(input: &str) -> bool {
    let mut first_char = input.chars().nth(0).unwrap();
    let mut second_char = input.chars().nth(1).unwrap();
    let mut third_char = input.chars().nth(2).unwrap();
    for c in input.chars().skip(3) {
        if first_char == third_char {
            return true;
        }
        first_char = second_char;
        second_char = third_char;
        third_char = c;
    }
    if first_char == third_char {
        return true;
    }
    false
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    let mut nice_strings: usize = 0;
    let mut really_nice_strings: usize = 0;
    for line in input.lines() {
        if vowel_check(line) && double_check(line) && no_bad_strings(line) {
            nice_strings += 1;
        }
        if non_overlapping_pairs(line) && repeat_with_interrupt(line) {
            really_nice_strings += 1;
        }
    }
    println!("nice strings: {0}", nice_strings);
    println!("nice strings for real this time: {0}", really_nice_strings);
}
