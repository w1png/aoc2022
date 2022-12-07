use std::{fs::File, io::Read};
use std::collections::HashMap;

fn part1(input: &String) {
    let mut dirs: HashMap<String, u32> = HashMap::new();
    let mut dir_tree: HashMap<String, HashMap<String, Vec<u64>>> = HashMap::new();
    let mut current_dir = vec!["/".to_string()];

    for (i, line) in input.lines().enumerate() {
        if line.starts_with("$ cd ") {
            let dir = line[5..].to_string();
            if dir == "/" {
                current_dir = vec!["/".to_string()];
            } else if dir == ".." {
                current_dir.pop();
            } else {
                current_dir.push(dir.to_string());
            }
        } else if line.starts_with("$ ls") {
            let mut j = i;
            let ls_line = input.lines().nth(j + 1).unwrap();
            while !ls_line.starts_with("$") {
                dbg!(&ls_line);

                if ls_line.starts_with("dir ") {
                    let dir = ls_line[4..].to_string();
                    current_dir.push(dir.to_string());
                    dir_tree.insert(current_dir.join("/"), HashMap::new());
                    current_dir.pop();
                } else {
                    dbg!(&line.split(" ").next().unwrap());
                    let size = line.split(" ").next().unwrap().to_string().parse::<u32>().unwrap();
                    let path = current_dir.join("/");
                    // add size to dir[path] += size or create new entry with size
                    if let Some(dir) = dirs.get_mut(&path) {
                        *dir += size;
                    } else {
                        dirs.insert(path, size);
                    }
                }
                if j+1 == input.lines().count() {
                    break;
                }
                let ls_line = input.lines().nth(j + 1).unwrap();
                j += 1;
            }

            
        }

        dbg!(&dirs, &dir_tree, &current_dir);
    }

    println!("{:?}", dirs);
}

fn part2(input: &String) {
    
}

fn main() {
    let mut input = String::new();
    File::open("test_input").unwrap().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}
