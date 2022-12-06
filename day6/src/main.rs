use std::{fs::File, io::Read};

fn part1(input: &String) {
    for i in 0..input.len()-3 {
        let mut chars: Vec<char> = input.chars().skip(i).take(4).collect();
        chars.sort();
        chars.dedup();
        if chars.len() == 4 {
            println!("Part 1: {}", i+4);
            break;
        }
    }
}

fn part2(input: &String) {
    for i in 0..input.len()-13 {
        let mut chars: Vec<char> = input.chars().skip(i).take(14).collect();
        chars.sort();
        chars.dedup();
        if chars.len() == 14 {
            println!("Part 2: {}", i+14);
            break;
        }
    }
}

fn main() {
    let mut input = String::new();
    File::open("input").unwrap().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}
