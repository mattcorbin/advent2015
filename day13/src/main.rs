use itertools::Itertools;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_line(line: &str) -> (String, String, i64) {
    let person1;
    let person2;
    let happiness_change;
    let mut parts = line.split(" would ");
    person1 = parts.next().unwrap().to_string();
    parts = parts
        .next()
        .unwrap()
        .split(" happiness units by sitting next to ");
    let mut raw_happiness = parts.next().unwrap().split(" ");
    person2 = parts.next().unwrap().trim_end_matches(".").to_string();
    let operator = raw_happiness.next().unwrap();
    let quantity = raw_happiness.next().unwrap().parse().unwrap();
    match operator {
        "gain" => happiness_change = quantity,
        "lose" => happiness_change = quantity * -1,
        _ => unreachable!(),
    }
    (person1, person2, happiness_change)
}

fn part1(input: &str) {
    let mut max_happiness = i64::MIN;
    let mut people = HashSet::new();
    let mut happiness_lookup: HashMap<String, HashMap<String, i64>> = HashMap::new();
    for line in input.lines() {
        let (person1, person2, happiness) = parse_line(line);
        people.insert(person1.clone());

        people.insert(person2.clone());

        if !happiness_lookup.contains_key(&person1) {
            let sub_map = HashMap::new();
            happiness_lookup.insert(person1.clone(), sub_map);
        }
        happiness_lookup
            .get_mut(&person1)
            .unwrap()
            .insert(person2.clone(), happiness);
    }
    for permutation in people.iter().permutations(people.len()).unique() {
        let mut total_happiness = 0;
        for i in 1..permutation.len() {
            total_happiness += happiness_lookup
                .get(permutation[i - 1])
                .unwrap()
                .get(permutation[i])
                .unwrap();
            total_happiness += happiness_lookup
                .get(permutation[i])
                .unwrap()
                .get(permutation[i - 1])
                .unwrap();
        }
        total_happiness += happiness_lookup
            .get(permutation[0])
            .unwrap()
            .get(permutation[permutation.len() - 1])
            .unwrap();
        total_happiness += happiness_lookup
            .get(permutation[permutation.len() - 1])
            .unwrap()
            .get(permutation[0])
            .unwrap();
        max_happiness = max(max_happiness, total_happiness);
    }
    println!("part1: {}", max_happiness);
}

fn part2(input: &str) {
    let mut max_happiness = i64::MIN;
    let mut people = HashSet::new();
    let mut happiness_lookup: HashMap<String, HashMap<String, i64>> = HashMap::new();
    let me = "me".to_string();
    people.insert(me.clone());
    happiness_lookup.insert(me.clone(), HashMap::new());
    for line in input.lines() {
        let (person1, person2, happiness) = parse_line(line);
        people.insert(person1.clone());

        people.insert(person2.clone());

        if !happiness_lookup.contains_key(&person1) {
            let sub_map = HashMap::new();
            happiness_lookup.insert(person1.clone(), sub_map);
        }
        happiness_lookup
            .get_mut(&person1)
            .unwrap()
            .insert(person2.clone(), happiness);
    }
    for person in &people {
        happiness_lookup
            .get_mut(person)
            .unwrap()
            .insert(me.clone(), 0);
        happiness_lookup
            .get_mut(&me)
            .unwrap()
            .insert(person.clone(), 0);
    }
    for permutation in people.iter().permutations(people.len()).unique() {
        let mut total_happiness = 0;
        for i in 1..permutation.len() {
            total_happiness += happiness_lookup
                .get(permutation[i - 1])
                .unwrap()
                .get(permutation[i])
                .unwrap();
            total_happiness += happiness_lookup
                .get(permutation[i])
                .unwrap()
                .get(permutation[i - 1])
                .unwrap();
        }
        total_happiness += happiness_lookup
            .get(permutation[0])
            .unwrap()
            .get(permutation[permutation.len() - 1])
            .unwrap();
        total_happiness += happiness_lookup
            .get(permutation[permutation.len() - 1])
            .unwrap()
            .get(permutation[0])
            .unwrap();
        max_happiness = max(max_happiness, total_happiness);
    }
    println!("part2: {}", max_happiness);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
