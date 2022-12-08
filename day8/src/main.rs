use std::{fs::File, io::Read};

fn part1(input: &String) {
    let mut trees: Vec<Vec<u32>> = vec![];
    let mut result = 0;

    for line in input.lines() {
        let mut row: Vec<u32> = vec![];
        for chr in line.chars() {
            row.push(chr.to_digit(10).unwrap());
        }
        trees.push(row);
    }

    result += (trees.len() + trees[0].len()) * 2 - 4;

    for (i, row) in trees.iter().enumerate() {
        if i == 0 { continue }
        if i == trees.len() - 1 { break }
        for (j, tree) in row.iter().enumerate() {
            if j == 0 { continue }
            if j == trees[0].len() - 1 { break }

            let mut top: Vec<i32> = vec![];
            for k in 0..i {
                top.push(trees[k][j] as i32);
            }

            let mut bottom: Vec<i32> = vec![];
            for k in i+1..trees.len() {
                bottom.push(trees[k][j] as i32);
            }

            let right = &trees[i][0..j];
            let left = &trees[i][j+1..];

            if top.iter().all(|&x| x < *tree as i32) || bottom.iter().all(|&x| x < *tree as i32) || right.iter().all(|&x| x < *tree) || left.iter().all(|&x| x < *tree) {
                result += 1;
            }
        }
    }

    println!("Part 1: {}", result);
}

fn part2(input: &String) {
    let mut trees: Vec<Vec<u32>> = vec![];
    let mut max_score = 0;

    for line in input.lines() {
        let mut row: Vec<u32> = vec![];
        for chr in line.chars() {
            row.push(chr.to_digit(10).unwrap());
        }
        trees.push(row);
    }

    for (i, row) in trees.iter().enumerate() {
        if i == 0 { continue }
        if i == trees.len() - 1 { break }
        for (j, tree) in row.iter().enumerate() {
            if j == 0 { continue }
            if j == trees[0].len() - 1 { break }

            let mut top: Vec<u32> = vec![];
            for k in 0..i {
                top.push(trees[k][j] as u32);
            }

            let mut bottom: Vec<u32> = vec![];
            for k in i+1..trees.len() {
                bottom.push(trees[k][j] as u32);
            }

            let mut right = trees.clone()[i][0..j].to_owned().to_vec();
            let left = trees.to_vec()[i][j+1..].to_owned().to_vec();

            right.reverse();
            top.reverse();

            fn process_score(values: Vec<u32>, curr: u32) -> u32 {
                let mut score = 0;
                for val in values.iter() {
                    score += 1;
                    if val >= &curr {
                        break;
                    }
                }
                score
            }

            let top_score = process_score(top, *tree);
            let bottom_score = process_score(bottom, *tree);
            let left_score = process_score(left, *tree);
            let right_score = process_score(right, *tree);

            let current_score = top_score * bottom_score * left_score * right_score;
            if current_score > max_score {
                max_score = current_score;
            }
        }
    }

    println!("Part 2: {}", max_score);
}
fn main() {
    let mut input = String::new();
    File::open("input").unwrap().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}

