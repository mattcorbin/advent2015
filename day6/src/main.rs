use std::fs;
use std::str::FromStr;

enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("turn off") {
            return Ok(Action::TurnOff);
        }
        else if s.starts_with("turn on") {
            return Ok(Action::TurnOn);
        }
        else if s.starts_with("toggle") {
            return Ok(Action::Toggle);
        }
        Err("invalid action!".to_string())
    }
}

fn main() {
    let mut lights: [[bool; 1000]; 1000] = [[false; 1000]; 1000];
    let mut dimmers: [[u8; 1000]; 1000] = [[0; 1000]; 1000];
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    for line in input.lines() {
        let action = Action::from_str(line).expect("should contain valid action");

        let splits = line.split(" ").filter(|item| item.contains(",")).collect::<Vec<&str>>();
        let start = splits[0].split(",").collect::<Vec<&str>>();
        let start_x = start[0].parse::<usize>().expect("should be a number");
        let start_y = start[1].parse::<usize>().expect("should be a number");

        let end = splits[1].split(",").collect::<Vec<&str>>();
        let end_x = end[0].parse::<usize>().expect("should be a number");
        let end_y = end[1].parse::<usize>().expect("should be a number");

        for i in start_x..=end_x {
            for j in start_y..=end_y {
                match action {
                    Action::TurnOn => {
                        lights[i][j] = true;
                        dimmers[i][j] += 1;
                    }
                    Action::TurnOff => {
                        lights[i][j] = false;
                        if dimmers[i][j] > 0 {
                            dimmers[i][j] -= 1;
                        }
                    }
                    Action::Toggle => {
                        lights[i][j] = !lights[i][j];
                        dimmers[i][j] += 2;
                    }
                }

            }
        }
    }
    let mut lit_lights: u64 = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if lights[i][j] {
                lit_lights += 1;
            }
        }
    }
    println!("total lights lit: {0}", lit_lights);
    let mut dimmer_brightness: u64 = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            dimmer_brightness += dimmers[i][j] as u64
        }
    }
    println!("total dimmer brightness: {0}", dimmer_brightness);
}
