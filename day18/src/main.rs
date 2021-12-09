use std::fs;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum LightState {
    On,
    Off,
}

impl Default for LightState {
    fn default() -> Self {
        LightState::Off
    }
}

impl FromStr for LightState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(LightState::On),
            "." => Ok(LightState::Off),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
struct Light {
    location: Point,
    current_state: LightState,
}

impl Light {
    fn new(s: &str, x: usize, y: usize) -> Light {
        Light {
            location: Point { x, y },
            current_state: LightState::from_str(s).unwrap(),
        }
    }

    fn neighbours(&self) -> Vec<Point> {
        let mut neighbours = Vec::new();
        let min_x;
        let max_x;
        let min_y;
        let max_y;
        match self.location.x {
            0 => {
                min_x = 0;
                max_x = self.location.x + 1;
            }
            99 => {
                min_x = self.location.x - 1;
                max_x = 99;
            }
            _ => {
                min_x = self.location.x - 1;
                max_x = self.location.x + 1;
            }
        }
        match self.location.y {
            0 => {
                min_y = 0;
                max_y = self.location.y + 1;
            }
            99 => {
                min_y = self.location.y - 1;
                max_y = 99;
            }
            _ => {
                min_y = self.location.y - 1;
                max_y = self.location.y + 1;
            }
        }
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if x == self.location.x && y == self.location.y {
                    continue;
                }
                neighbours.push(Point { x, y })
            }
        }
        neighbours
    }
}

fn get_lit_neighbour_count(graph: &[[Light; 100]; 100], x: usize, y: usize) -> usize {
    let mut neighbours_on = 0;
    for state in graph[x][y].neighbours().iter().map(|z| graph[z.x][z.y].current_state) {
        if state == LightState::On {
            neighbours_on += 1;
        }
    }
    neighbours_on
}

fn get_new_state(graph: &[[Light; 100]; 100], x: usize, y: usize, neighbours_on: usize) -> LightState {
    let mut new_state = LightState::Off;
    match graph[x][y].current_state {
        LightState::On => {
            if neighbours_on == 2 || neighbours_on == 3 {
                new_state = LightState::On;
            }
        }
        LightState::Off => {
            if neighbours_on == 3 {
                new_state = LightState::On
            }
        }
    }
    new_state
}

fn part1(input: &str) {
    let mut graph: [[Light; 100]; 100] = [[Light::default(); 100]; 100];
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            let light = Light::new(&c.to_string(), x, y);
            graph[x][y] = light;
        }
    }
    for _ in 0..100 {
        let mut new_graph: [[Light; 100]; 100] = [[Light::default(); 100]; 100];
        for x in 0..100 {
            for y in 0..100 {
                let neighbours_on = get_lit_neighbour_count(&graph, x, y);
                let new_state = get_new_state(&graph, x, y, neighbours_on);
                let new_light = Light {
                    location: Point { x, y },
                    current_state: new_state,
                };
                new_graph[x][y] = new_light;
            }
        }
        graph = new_graph;
    }
    let mut lights_on = 0;
    for row in graph {
        for light in row {
            if light.current_state == LightState::On {
                lights_on += 1;
            }
        }
    }
    println!("part1: {}", lights_on)
}

fn part2(input: &str) {
    let mut graph: [[Light; 100]; 100] = [[Light::default(); 100]; 100];
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            let light = Light::new(&c.to_string(), x, y);
            graph[x][y] = light;
        }
    }
    for _ in 0..100 {
        let mut new_graph: [[Light; 100]; 100] = [[Light::default(); 100]; 100];
        for x in 0..100 {
            for y in 0..100 {
                let neighbours_on = get_lit_neighbour_count(&graph, x, y);
                let new_state = get_new_state(&graph, x, y, neighbours_on);
                let new_light = Light {
                    location: Point { x, y },
                    current_state: new_state,
                };
                new_graph[x][y] = new_light;
            }
        }
        new_graph[0][0].current_state = LightState::On;
        new_graph[0][99].current_state = LightState::On;
        new_graph[99][0].current_state = LightState::On;
        new_graph[99][99].current_state = LightState::On;
        graph = new_graph;
    }
    let mut lights_on = 0;
    for row in graph {
        for light in row {
            if light.current_state == LightState::On {
                lights_on += 1;
            }
        }
    }
    println!("part2: {}", lights_on)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
