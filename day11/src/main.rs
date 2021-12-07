use std::fs;

fn contains_run(input: &str) -> bool {
    let bytes = input.as_bytes();
    for i in 2..bytes.len() {
        if bytes[i-2] + 1 == bytes[i-1] && bytes[i-1] + 1 == bytes[i] {
            return true;
        }
    }
    false
}

fn contains_no_bad_characters(input: &str) -> bool {
    !input.contains(&['i', 'o', 'l'][..])
}

fn contains_non_overlapping_pairs(input: &str) -> bool {
    let mut first_pair = '0';
    let mut found = false;
    for c in 'a'..='z' {
        let test = format!("{}{}", c, c);
        if input.contains(&test) {
            first_pair = c;
            found = true;
            break;
        }
    }
    if !found {
        return false;
    }
    for c in 'a'..='z' {
        if c == first_pair {
            continue;
        }
        let test = format!("{}{}", c, c);
        if input.contains(&test) {
            return true;
        }
    }
    false
}

fn is_valid_password(password: &str) -> bool {
    contains_run(password) && contains_no_bad_characters(password) && contains_non_overlapping_pairs(password)
}

fn next_password(input: &str) -> String {
    let a = 'a' as u8;
    let z = 'z' as u8;
    let mut bytes = input.as_bytes().to_vec();
    for i in (0..bytes.len()).rev() {
        if bytes[i] == z {
            bytes[i] = a;
        } else {
            bytes[i] += 1;
            break
        }
    }
    String::from_utf8(bytes).unwrap()
}

fn part1(input: &str) -> String {
    let mut new_password = next_password(input);
    loop {
        if is_valid_password(&new_password) {
            break;
        }
        new_password = next_password(&new_password);
    }
    println!("part1: {}", new_password);
    new_password
}

fn part2(input: &str) {
    let mut new_password = next_password(input);
    loop {
        if is_valid_password(&new_password) {
            break;
        }
        new_password = next_password(&new_password);
    }
    println!("part2: {}", new_password);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part2(&part1(&input));
}
