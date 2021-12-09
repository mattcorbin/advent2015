use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Default, Debug, Ord, PartialOrd, Copy, Clone)]
struct AuntSue {
    id: u64,
    children: Option<u64>,
    cats: Option<u64>,
    samoyeds: Option<u64>,
    pomeranians: Option<u64>,
    akitas: Option<u64>,
    vizslas: Option<u64>,
    goldfish: Option<u64>,
    trees: Option<u64>,
    cars: Option<u64>,
    perfumes: Option<u64>,
}

impl AuntSue {
    fn new(
        children: u64,
        cats: u64,
        samoyeds: u64,
        pomeranians: u64,
        akitas: u64,
        vizslas: u64,
        goldfish: u64,
        trees: u64,
        cars: u64,
        perfumes: u64,
    ) -> AuntSue {
        AuntSue {
            id: 0,
            children: Some(children),
            cats: Some(cats),
            samoyeds: Some(samoyeds),
            pomeranians: Some(pomeranians),
            akitas: Some(akitas),
            vizslas: Some(vizslas),
            goldfish: Some(goldfish),
            trees: Some(trees),
            cars: Some(cars),
            perfumes: Some(perfumes),
        }
    }

    fn set_field(&mut self, field_name: &str, field_value: u64) {
        match field_name {
            "children" => self.children = Some(field_value),
            "cats" => self.cats = Some(field_value),
            "samoyeds" => self.samoyeds = Some(field_value),
            "pomeranians" => self.pomeranians = Some(field_value),
            "akitas" => self.akitas = Some(field_value),
            "vizslas" => self.vizslas = Some(field_value),
            "goldfish" => self.goldfish = Some(field_value),
            "trees" => self.trees = Some(field_value),
            "cars" => self.cars = Some(field_value),
            "perfumes" => self.perfumes = Some(field_value),
            _ => unreachable!(),
        }
    }

    fn kinda_eq(&self, other: &Self) -> bool {
        let mut comps: Vec<bool> = Vec::new();
        if let Some(a) = self.children {
            if let Some(b) = other.children {
                comps.push(a == b);
            }
        }
        if let Some(a) = self.cats {
            if let Some(b) = other.cats {
                comps.push(a > b);
            }
        }
        if let Some(a) = self.samoyeds {
            if let Some(b) = other.samoyeds {
                comps.push(a == b);
            }
        }
        if let Some(a) = self.pomeranians {
            if let Some(b) = other.pomeranians {
                comps.push(a < b);
            }
        }
        if let Some(a) = self.akitas {
            if let Some(b) = other.akitas {
                comps.push(a == b);
            }
        }
        if let Some(a) = self.vizslas {
            if let Some(b) = other.vizslas {
                comps.push(a == b);
            }
        }
        if let Some(a) = self.goldfish {
            if let Some(b) = other.goldfish {
                comps.push(a < b);
            }
        }
        if let Some(a) = self.trees {
            if let Some(b) = other.trees {
                comps.push(a > b);
            }
        }
        if let Some(a) = self.cars {
            if let Some(b) = other.cars {
                comps.push(a == b);
            }
        }
        if let Some(a) = self.perfumes {
            if let Some(b) = other.perfumes {
                comps.push(a == b);
            }
        }
        comps.len() > 0 && comps.into_iter().all(|x| x)
    }
}

impl FromStr for AuntSue {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.splitn(2, ":");
        let id = splits.next().unwrap().trim_start_matches("Sue ").parse()?;
        let mut sue = AuntSue {
            id,
            ..Default::default()
        };
        for split in splits.next().unwrap().split(",") {
            let pieces = split.trim_start_matches(" ").split(": ").collect::<Vec<&str>>();
            sue.set_field(pieces[0], pieces[1].parse()?);
        }
        Ok(sue)
    }
}

impl PartialEq<Self> for AuntSue {
    fn eq(&self, other: &Self) -> bool {
        let mut comps: Vec<bool> = Vec::new();
        if let Some(a) = self.children {
            if let Some(b) = other.children {
                comps.push(a == b);
            }
        }
        if let Some(a) = self.cats {
            if let Some(b) = other.cats {
                comps.push(a == b);
            }
        }
        if let Some(a) = self.samoyeds {
            if let Some(b) = other.samoyeds {
                comps.push(a == b);
            }
        }
        if let Some(a) = self.pomeranians {
            if let Some(b) = other.pomeranians {
                comps.push(a == b);
            }
        }
        if let Some(a) = self.akitas {
            if let Some(b) = other.akitas {
                comps.push(a == b);
            }
        }
        if let Some(a) = self.vizslas {
            if let Some(b) = other.vizslas {
                comps.push(a == b);
            }
        }
        if let Some(a) = self.goldfish {
            if let Some(b) = other.goldfish {
                comps.push(a == b);
            }
        }
        if let Some(a) = self.trees {
            if let Some(b) = other.trees {
                comps.push(a == b);
            }
        }
        if let Some(a) = self.cars {
            if let Some(b) = other.cars {
                comps.push(a == b);
            }
        }
        if let Some(a) = self.perfumes {
            if let Some(b) = other.perfumes {
                comps.push(a == b);
            }
        }
        comps.len() > 0 && comps.into_iter().all(|x| x)
    }
}

impl Eq for AuntSue {}

fn part1(input: &str) {
    let mut all_sues = Vec::new();
    for line in input.lines() {
        let sue = AuntSue::from_str(line).unwrap();
        all_sues.push(sue);
    }
    let desired_sue = AuntSue::new(3, 7, 2, 3, 0, 0, 5, 3, 2, 1);
    let mut right_sue = 0;
    for sue in all_sues {
        if sue == desired_sue {
            right_sue = sue.id;
        }
    }
    println!("part1: {}", right_sue)
}

fn part2(input: &str) {
    let mut all_sues = Vec::new();
    for line in input.lines() {
        let sue = AuntSue::from_str(line).unwrap();
        all_sues.push(sue);
    }
    let desired_sue = AuntSue::new(3, 7, 2, 3, 0, 0, 5, 3, 2, 1);
    let mut right_sue = 0;
    for sue in all_sues {
        if sue.kinda_eq(&desired_sue) {
            right_sue = sue.id;
        }
    }
    println!("part2: {}", right_sue)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
