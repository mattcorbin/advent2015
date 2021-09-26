use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Location {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Direction::Up),
            "v" => Ok(Direction::Down),
            "<" => Ok(Direction::Left),
            ">" => Ok(Direction::Right),
            _ => Err("Invalid direction!".to_string())
        }
    }
}

impl Location {
    fn calculate_move(&mut self, direction: Direction) -> Location {
        let mut new_location = Location{x: self.x, y: self.y};
        match direction {
            Direction::Up => {new_location.y += 1},
            Direction::Down => {new_location.y -= 1},
            Direction::Left => {new_location.x -= 1},
            Direction::Right => {new_location.x += 1},
        }
        new_location
    }
}

fn main() {
    let mut visited_locations: HashSet<Location> = HashSet::new();
    let mut p2_visited_locations: HashSet<Location> = HashSet::new();
    let mut current_location = Location{x: 0, y: 0};
    let mut santa_location = Location{x: 0, y: 0};
    let mut robo_santa_location = Location{x: 0, y: 0};
    visited_locations.insert(current_location);
    p2_visited_locations.insert(current_location);
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    for (idx, c) in input.chars().enumerate() {
        let char_string = c.to_string();
        let direction = Direction::from_str(&char_string).expect("should be valid direction");
        let new_location = current_location.calculate_move(direction);
        visited_locations.insert(new_location);
        current_location = new_location;

        match idx%2 {
            0 => {
                let new_location = santa_location.calculate_move(direction);
                p2_visited_locations.insert(new_location);
                santa_location = new_location;
            },
            _ => {
                let new_location = robo_santa_location.calculate_move(direction);
                p2_visited_locations.insert(new_location);
                robo_santa_location = new_location;
            }
        }
    }

    println!("Houses visited: {0}", visited_locations.len());
    println!("Houses visited p2: {0}", p2_visited_locations.len());
}
