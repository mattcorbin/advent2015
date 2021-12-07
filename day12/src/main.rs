use std::fs;
use serde_json::{Value};

fn walk_json(input: Value) -> i64 {
    match input {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(a) => a.iter().map(|x| walk_json(x.clone())).sum(),
        Value::Object(o) =>  o.values().map(|x| walk_json(x.clone())).sum(),
    }
}

fn part1(input: &str) {
    let garbage: Value = serde_json::from_str(input).unwrap();
    let sum = walk_json(garbage);
    println!("part1: {}", sum);
}

fn walk_json_no_red(input: Value) -> i64 {
    match input {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(a) => a.iter().map(|x| walk_json_no_red(x.clone())).sum(),
        Value::Object(o) => {
            if o.values().any(|x| x.is_string() && x.as_str().unwrap() == "red") {
                0
            } else {
                o.values().map(|x| walk_json_no_red(x.clone())).sum()
            }
        },
    }
}


fn part2(input: &str) {
    let garbage: Value = serde_json::from_str(input).unwrap();
    let sum = walk_json_no_red(garbage);
    println!("part2: {}", sum);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
