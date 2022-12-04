use std::{fs::File, io::Read};

fn part1(input: &String) {
    let mut result = 0;
    for line in input.lines() {
        let elves = line.split(",").collect::<Vec<&str>>();
        let elf1 = elves[0].split("-").collect::<Vec<&str>>().iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let elf2 = elves[1].split("-").collect::<Vec<&str>>().iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        if elf1[0] <= elf2[0] && elf2[1] <= elf1[1] || elf2[0] <= elf1[0] && elf1[1] <= elf2[1] {
            result += 1;
        }
    }
    println!("Part 1: {}", result);
}

fn part2(input: &String) {
    let mut result = 0;
    for line in input.lines() {
        let elves = line.split(",").collect::<Vec<&str>>();
        let elf1 = elves[0].split("-").collect::<Vec<&str>>().iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let elf2 = elves[1].split("-").collect::<Vec<&str>>().iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        if elf1[0] <= elf2[0] && elf2[1] <= elf1[1] || elf2[0] <= elf1[0] && elf1[1] <= elf2[1] {
            result += 1;
        } else if elf1[0] <= elf2[0] && elf2[0] <= elf1[1] || elf2[0] <= elf1[0] && elf1[0] <= elf2[1] {
            result += 1;
        }
    }

    println!("Part 2: {}", result);
}

fn main() {
    let mut input = String::new();
    File::open("input").unwrap().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}
