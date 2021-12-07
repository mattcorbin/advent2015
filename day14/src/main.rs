use std::cmp::max;
use std::fs;
use std::fs::read;
use std::str::FromStr;

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: u64,
    duration: u64,
    rest: u64,
    distance_travelled: u64,
    points: u64,
}

impl Reindeer {}

impl FromStr for Reindeer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name;
        let speed;
        let duration;
        let rest;
        let mut parts = s.split(" can fly ");
        name = parts.next().unwrap().to_string();
        let mut parts = parts.next().unwrap().split(" km/s for ");
        speed = parts.next().unwrap().parse().unwrap();
        let mut parts = parts
            .next()
            .unwrap()
            .split(" seconds, but then must rest for ");
        duration = parts.next().unwrap().parse().unwrap();
        let mut parts = parts.next().unwrap().split(" seconds.");
        rest = parts.next().unwrap().parse().unwrap();
        Ok(Reindeer {
            name,
            speed,
            duration,
            rest,
            distance_travelled: 0,
            points: 0,
        })
    }
}

fn part1(input: &str) {
    let mut all_reindeer = Vec::new();
    for line in input.lines() {
        all_reindeer.push(Reindeer::from_str(line).unwrap());
    }
    let mut winner = 0;
    for reindeer in all_reindeer {
        let mut distance_travelled = 0;
        let total_duration = reindeer.duration + reindeer.rest;
        for i in 0..2503 {
            if i % total_duration < reindeer.duration {
                distance_travelled += reindeer.speed
            }
        }
        winner = max(winner, distance_travelled);
    }
    println!("part1: {}", winner)
}

fn part2(input: &str) {
    let mut all_reindeer = Vec::new();
    for line in input.lines() {
        all_reindeer.push(Reindeer::from_str(line).unwrap());
    }
    for i in 0..2503 {
        for reindeer in all_reindeer.iter_mut() {
            let total_duration = reindeer.duration + reindeer.rest;
            if i % total_duration < reindeer.duration {
                reindeer.distance_travelled += reindeer.speed;
            }
        }
        let in_the_lead = all_reindeer.iter().map(|x| x.distance_travelled).max().unwrap();
        for reindeer in all_reindeer.iter_mut() {
            if reindeer.distance_travelled == in_the_lead {
                reindeer.points += 1
            }
        }
    }
    println!("part2: {}", all_reindeer.iter().map(|x| x.points).max().unwrap())
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
