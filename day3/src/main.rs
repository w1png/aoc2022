use std::{fs::File, io::Read};

fn part1(input: &String) {
    let letters = "abcdefghijklmnopqrstuvwxyz".to_string();
    let mut result = 0;
    for line in input.lines() {
        let first_half = line[..line.len() / 2].to_string();
        let second_half = line[line.len() / 2..].to_string();

        for chr in first_half.chars() {
            if second_half.contains(chr) {
                result += letters.find(chr.to_lowercase().to_string().as_str()).unwrap() + 1;
                if chr.to_uppercase().to_string() == chr.to_string() {
                    result += 26;
                }
                break;
            }
        }
    }
    println!("Part 1: {}", result);
}

fn part2(input: &String) {
    let letters = "abcdefghijklmnopqrstuvwxyz".to_string();
    let mut result = 0;
    
    let mut i = 0;
    while i + 3 <= input.lines().count() {
        let line1 = input.lines().nth(i).unwrap();
        let line2 = input.lines().nth(i + 1).unwrap();
        let line3 = input.lines().nth(i + 2).unwrap();

        for ch in line1.chars() {
            if line2.contains(ch) && line3.contains(ch) {
                result += letters.find(ch.to_lowercase().to_string().as_str()).unwrap() + 1;
                if ch.to_uppercase().to_string() == ch.to_string() {
                    result += 26;
                }
                break;
            }
        }
        i += 3;
    }

    println!("Part 2: {}", result);

}

fn main() {
    let mut input = String::new();
    File::open("input").unwrap().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}
