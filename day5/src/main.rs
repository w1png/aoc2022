use std::{fs::File, io::Read};

fn part1(input: &String) {
    let mut moves_index = 0;
    let mut columns: Vec<Vec<char>> = Vec::new();
    
    for (i, line) in input.lines().enumerate() {
        if line.starts_with(" 1") {
            moves_index = i;
            break;
        }

        let mut j = 0;
        let mut current_col = 0;
        loop {
            if j+2 >= line.len() {
                break;
            }
            let current_char = line.chars().nth(j+1).unwrap();
            if current_char != ' ' {
                if columns.len() <= current_col {
                    columns.push(vec![current_char]);
                } else {
                    columns[current_col].insert(0, current_char);
                }
            } else if columns.len() <= current_col {
                columns.push(vec![]);
                
            }

            j += 4;
            current_col += 1;
        }
    }

    let moves = input.lines().skip(moves_index+2).collect::<Vec<&str>>();
    for mv in moves {
        let move_amount = mv.split(" ").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
        let from = mv.split(" ").collect::<Vec<&str>>()[3].parse::<usize>().unwrap();
        let to = mv.split(" ").collect::<Vec<&str>>()[5].parse::<usize>().unwrap();

        
        for _ in 0..move_amount {
            let chr = columns[from-1].pop().unwrap();
            columns[to-1].push(chr);
        }
    }

    let result = columns.iter().map(|c| {
        c.last().unwrap().to_string()
    }).collect::<Vec<String>>().join("");
    

    println!("Part 1: {}", result);
}

fn part2(input: &String) {
    let mut moves_index = 0;
    let mut columns: Vec<Vec<char>> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        if line.starts_with(" 1") {
            moves_index = i;
            break;
        }

        let mut j = 0;
        let mut current_col = 0;
        loop {
            if j+2 >= line.len() {
                break;
            }
            let current_char = line.chars().nth(j+1).unwrap();
            if current_char != ' ' {
                if columns.len() <= current_col {
                    columns.push(vec![current_char]);
                } else {
                    columns[current_col].insert(0, current_char);
                }
            } else if columns.len() <= current_col {
                columns.push(vec![]);
                
            }

            j += 4;
            current_col += 1;
        }
    }

    let moves = input.lines().skip(moves_index+2).collect::<Vec<&str>>();
    for mv in moves {
        let move_amount = mv.split(" ").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
        let from = mv.split(" ").collect::<Vec<&str>>()[3].parse::<usize>().unwrap();
        let to = mv.split(" ").collect::<Vec<&str>>()[5].parse::<usize>().unwrap();

        if move_amount == 1 {
            let chr = columns[from-1].pop().unwrap();
            columns[to-1].push(chr);
        } else {
            let from_len = columns[from-1].len();
            let tmp = &columns.clone()[from-1][from_len-move_amount..];
            for item in tmp {
                columns[to-1].push(*item);
            }

            columns[from-1].truncate(from_len-move_amount);
        }
    }

    let result = columns.iter().map(|c| {
        c.last().unwrap().to_string()
    }).collect::<Vec<String>>().join("");
    

    println!("Part 2: {}", result);
}
fn main() {
    let mut input = String::new();
    File::open("input").unwrap().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}
