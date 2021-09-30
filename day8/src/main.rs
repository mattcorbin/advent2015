use std::fs;

fn count_characters(line: &str) -> usize {
    let mut count = 0;
    let mut escape_next = false;
    let mut skip = 0;
    for c in line.chars() {
        if skip > 0 {
            skip -= 1;
        } else if escape_next {
            escape_next = false;
            if c == 'x' {
                skip = 2
            }
        } else if c == '\\' {
            escape_next = true;
            count += 1;
        } else {
            count += 1;
        }
    }
    count
}

fn escape_again(line: &str) -> usize {
    line.replace("\\", "\\\\").replace("\"","\\\"").len() + 2
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    let mut total_chars = 0;
    let mut memory_values = 0;
    let mut double_escaped = 0;
    for line in input.lines() {
        total_chars += line.len();
        memory_values += count_characters(line.trim_matches('\"'));
        double_escaped += escape_again(line);
    }
    println!("difference in characters p1: {0}", total_chars - memory_values);
    println!("difference in characters p2: {0}", double_escaped - total_chars);
}
