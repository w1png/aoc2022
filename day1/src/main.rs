use std::{fs::File, io::Read};

fn part1(input: &String) {
    let lines = input.split("\n");

    let mut max_elf = 0;
    let mut current_elf = 0;
    for line in lines {
        let num = line.parse::<i32>();
        if num.is_err() {
            if current_elf > max_elf {
                max_elf = current_elf;
            }
            current_elf = 0;
            continue;
        }
        current_elf += num.unwrap();
    }

    println!("{}", max_elf);
}

fn part2(input: &String) {
    let lines = input.split("\n");

    let mut elfs = Vec::new();
    let mut current_elf = 0;
    for line in lines {
        let num = line.parse::<i32>();
        if num.is_err() {
            elfs.push(current_elf);
            current_elf = 0;
            continue;
        }
        current_elf += num.unwrap();
    }
    elfs.sort();
    elfs.reverse();

    println!("{}", elfs[0] + elfs[1] + elfs[2]);
}

fn main() {
    let mut input = String::new();
    File::open("input").unwrap().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}
