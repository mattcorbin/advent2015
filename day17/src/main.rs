use std::fs;
use itertools::Itertools;

fn part1(input: &str) {
    let mut containers: Vec<u64> = Vec::new();
    for line in input.lines() {
        containers.push(line.parse().unwrap());
    }
    let mut combinations = 0;
    for set in containers.into_iter().powerset() {
        if set.iter().sum::<u64>() == 150 {
            combinations += 1;
        }
    }
    println!("part1: {}", combinations)
}

fn part2(input: &str) {
    let mut containers: Vec<u64> = Vec::new();
    for line in input.lines() {
        containers.push(line.parse().unwrap());
    }
    let mut combinations = 0;
    let mut min_containers: usize = usize::MAX;
    for set in containers.into_iter().powerset() {
        if set.iter().sum::<u64>() == 150 {
            if set.len() < min_containers {
                min_containers = set.len();
                combinations = 1;
            } else if set.len() == min_containers {
                combinations += 1;
            }
        }
    }
    println!("part2: {}", combinations)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
