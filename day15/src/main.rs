use std::cmp::max;
use std::fs;
use std::str::FromStr;

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavour: i64,
    texture: i64,
    calories: i64,
}

impl FromStr for Ingredient {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name;
        let capacity;
        let durability;
        let flavour;
        let texture;
        let calories;
        let mut parts = s.split(&[',', ':'][..]);
        name = parts.next().unwrap().to_string();
        capacity = parts
            .next()
            .unwrap()
            .trim_start_matches(" capacity ")
            .parse()
            .unwrap();
        durability = parts
            .next()
            .unwrap()
            .trim_start_matches(" durability ")
            .parse()
            .unwrap();
        flavour = parts
            .next()
            .unwrap()
            .trim_start_matches(" flavor ")
            .parse()
            .unwrap();
        texture = parts
            .next()
            .unwrap()
            .trim_start_matches(" texture ")
            .parse()
            .unwrap();
        calories = parts
            .next()
            .unwrap()
            .trim_start_matches(" calories ")
            .parse()
            .unwrap();
        Ok(Ingredient {
            name,
            capacity,
            durability,
            flavour,
            texture,
            calories,
        })
    }
}

fn part1(input: &str) {
    let mut ingredients = Vec::new();
    for line in input.lines() {
        ingredients.push(Ingredient::from_str(line).unwrap());
    }
    let mut max_score = i64::MIN;
    for i in 0..=100 {
        for j in 0..=100 - i {
            for k in 0..=100 - i - j {
                for l in 0..=100 - i - j - k {
                    if (i + j + k + l) != 100 {
                        continue;
                    }
                    let mut capacity = ingredients[0].capacity * i
                        + ingredients[1].capacity * j
                        + ingredients[2].capacity * k
                        + ingredients[3].capacity * l;
                    let mut durability = ingredients[0].durability * i
                        + ingredients[1].durability * j
                        + ingredients[2].durability * k
                        + ingredients[3].durability * l;
                    let mut flavour = ingredients[0].flavour * i
                        + ingredients[1].flavour * j
                        + ingredients[2].flavour * k
                        + ingredients[3].flavour * l;
                    let mut texture = ingredients[0].texture * i
                        + ingredients[1].texture * j
                        + ingredients[2].texture * k
                        + ingredients[3].texture * l;
                    if capacity < 0 {
                        capacity = 0;
                    }
                    if durability < 0 {
                        durability = 0;
                    }
                    if flavour < 0 {
                        flavour = 0;
                    }
                    if texture < 0 {
                        texture = 0;
                    }
                    let score = capacity * durability * flavour * texture;
                    max_score = max(max_score, score);
                }
            }
        }
    }
    println!("part1: {}", max_score)
}

fn part2(input: &str) {
    let mut ingredients = Vec::new();
    for line in input.lines() {
        ingredients.push(Ingredient::from_str(line).unwrap());
    }
    let mut max_score = i64::MIN;
    for i in 0..=100 {
        for j in 0..=100 - i {
            for k in 0..=100 - i - j {
                for l in 0..=100 - i - j - k {
                    if (i + j + k + l) != 100 {
                        continue;
                    }
                    let mut capacity = ingredients[0].capacity * i
                        + ingredients[1].capacity * j
                        + ingredients[2].capacity * k
                        + ingredients[3].capacity * l;
                    let mut durability = ingredients[0].durability * i
                        + ingredients[1].durability * j
                        + ingredients[2].durability * k
                        + ingredients[3].durability * l;
                    let mut flavour = ingredients[0].flavour * i
                        + ingredients[1].flavour * j
                        + ingredients[2].flavour * k
                        + ingredients[3].flavour * l;
                    let mut texture = ingredients[0].texture * i
                        + ingredients[1].texture * j
                        + ingredients[2].texture * k
                        + ingredients[3].texture * l;
                    let calories = ingredients[0].calories * i
                        + ingredients[1].calories * j
                        + ingredients[2].calories * k
                        + ingredients[3].calories * l;
                    if capacity < 0 {
                        capacity = 0;
                    }
                    if durability < 0 {
                        durability = 0;
                    }
                    if flavour < 0 {
                        flavour = 0;
                    }
                    if texture < 0 {
                        texture = 0;
                    }
                    let score = capacity * durability * flavour * texture;
                    if calories == 500 {
                        max_score = max(max_score, score);
                    }
                }
            }
        }
    }
    println!("part2: {}", max_score)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
