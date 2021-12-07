extern crate itertools;

use std::fs;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn parse_line(line: &str) -> (String, String, u64) {
    let parts = line.replace(" to ", " ");
    let parts = parts.replace(" = ", " ");
    let parts = parts.split(" ").collect::<Vec<&str>>();
    let city1 = parts[0].to_string();
    let city2 = parts[1].to_string();
    let distance = parts[2].parse().expect("final part should be an integer");
    (city1, city2, distance)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    let mut shortest = u64::MAX;
    let mut longest = 0;
    let mut places = HashSet::new();
    let mut distances: HashMap<String, HashMap<String, u64>> = HashMap::new();
    for line in input.lines() {
        let (city1, city2, distance) = parse_line(line);
        places.insert(city1.clone());
        places.insert(city2.clone());

        if !distances.contains_key(&city1) {
            let sub_map = HashMap::new();
            distances.insert(city1.clone(), sub_map);
        }
        if !distances.contains_key(&city2) {
            let sub_map = HashMap::new();
            distances.insert(city2.clone(), sub_map);
        }
        distances.get_mut(&city1).unwrap().insert(city2.clone(), distance);
        distances.get_mut(&city2).unwrap().insert(city1.clone(), distance);
    }
    for permutation in places.iter().permutations(places.len()).unique() {
        let mut dist = 0;
        for i in 1..permutation.len() {
            dist += distances.get(permutation[i-1]).unwrap().get(permutation[i]).unwrap()
        }
        shortest = min(shortest, dist);
        longest = max(longest, dist);
    }
    println!("part1: {}", shortest);
    println!("part2: {}", longest);
}
