use std::{fs::File, io::Read, collections::HashMap};

fn calculate_score_p1(char1: char, char2: char) -> i32 {
    let mut score = 0;
    let mut char2 = char2;
    match char2 {
        'X' => {
            char2 = 'A';
            score += 1;
        },
        'Y' => {
            char2 = 'B';
            score += 2;
        },
        'Z' => {
            char2 = 'C';
            score += 3;
        },
        _ => (),
    }

    if char1 == char2 {
        score += 3;
    } else if char2 == 'A' && char1 == 'C' {
        score += 6;
    } else if char2 == 'B' && char1 == 'A' {
        score += 6;
    } else if char2 == 'C' && char1 == 'B' {
        score += 6;
    }
    score
}

fn part1(input: &String) {
    let mut score = 0;
    for line in input.lines() {
        let chars = line.chars();
        let char1 = chars.clone().nth(0).unwrap();
        let char2 = chars.clone().nth(2).unwrap();
        score += calculate_score_p1(char1, char2);
    }
    println!("Part 1: {}", score);
}

fn calculate_score_p2(char1: char, char2: char) -> i32 {
    let mut score = 0;
    let scores_per_char = HashMap::from([
        ('A', 1),
        ('B', 2),
        ('C', 3),
    ]);

    match char2 {
        'Y' => {
            score += scores_per_char.get(&char1).unwrap();
            score += 3;
        },
        'Z' => {
            match char1 {
                'A' => score += 2,
                'B' => score += 3,
                'C' => score += 1,
                _ => (),
            }
            score += 6;
        },
        'X' => {
            match char1 {
                'A' => score += 3,
                'B' => score += 1,
                'C' => score += 2,
                _ => (),
            }
        }
        _ => (),
    }


    score
}

fn part2(input: &String) {
    let mut score = 0;
    for line in input.lines() {
        let chars = line.chars();
        let char1 = chars.clone().nth(0).unwrap();
        let char2 = chars.clone().nth(2).unwrap();
        score += calculate_score_p2(char1, char2);
    }
    println!("Part 1: {}", score);
}

fn main() {
    let mut input = String::new();
    File::open("input").unwrap().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}
