use std::fs;

fn main() {
    let mut entered_basement = false;
    let mut floor: i64 = 0;
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    for (idx, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => println!("unrecognized char {0}", c)
        }
        if floor < 0 && !entered_basement {
            entered_basement = true;
            println!("char {0} enters the basement", idx + 1);
        }
    }
    println!("floor: {0}", floor);
}
