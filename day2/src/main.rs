use std::fs;
use std::cmp;
use std::io::{BufReader, BufRead};

fn main() {
    let mut required_paper: u64 = 0;
    let mut required_ribbon: u64 = 0;
    let file = fs::File::open("input.txt").expect("input file should exist");

    for l in BufReader::new(file).lines() {
        let line = l.expect("valid line");
        let split = line.split("x").collect::<Vec<&str>>();
        let dimensions = split
            .iter()
            .map(|item| item.to_string().parse::<u64>().expect("should be a valid int"))
            .collect::<Vec<u64>>();
        let length = dimensions[0];
        let width = dimensions[1];
        let height = dimensions[2];
        let surface_area = 2*length*width + 2*width*height + 2*height*length;
        required_paper += surface_area;
        required_paper +=  cmp::min(cmp::min(length*width, width*height), height*length);

        required_ribbon += cmp::min(cmp::min(2*length + 2*width, 2*width + 2*height), 2*height + 2*length);
        required_ribbon += length*width*height;
    }
    println!("Required paper: {0}", required_paper);
    println!("Required ribbon: {0}", required_ribbon);
}
