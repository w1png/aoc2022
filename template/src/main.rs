use std::{fs::File, io::Read};

fn part1(input: &String) {
    
}

fn part2(input: &String) {
    
}

fn main() {
    let mut input = String::new();
    File::open("input").unwrap().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}
