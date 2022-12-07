static INPUT: &'static str = include_str!("input/day07.txt");

use std::collections::HashMap;

pub fn silver_and_gold() {
    let mut dir_stack: Vec<String> = Vec::new();
    let mut sizes: HashMap<String, usize> = HashMap::new();

    for line in INPUT.lines() {
        if line.starts_with("$ cd") {
            let dest_dir = &line[5..];

            // Handle command
            match dest_dir {
                "/" => {
                    dir_stack = vec!["/".into()] // Reset
                },
                ".." => {
                    dir_stack.pop().unwrap();
                },
                _ => {
                    dir_stack.push(dest_dir.into());
                }
            }
        } else if line.starts_with("$ ls") {
            // noise
        } else {
            // currently in directory `cur`
            let (size, _filename) = line.split_once(' ').unwrap();
            if size == "dir" {
                continue;
            } else {
                let size = size.parse::<usize>().unwrap();
                for i in 0..dir_stack.len() {
                    *sizes.entry(dir_stack[..=i].concat()).or_default() += size;
                }
            }
        }
    }

    let silver = sizes.iter()
        .filter(|(_, &size)| size <= 100_000)
        .fold(0 as usize, |acc, (_, &size)| acc + size);

    let used: &usize = sizes.get("/".into()).unwrap(); // Total usage
    let target: usize = (used + 30_000_000) - 70_000_000; // How much space we need at minimum

    let gold = sizes.iter()
        .filter_map(|(_, &size)| if size >= target { Some(size) } else { None })
        .min()
        .unwrap();

    println!("silver: {}", silver);
    println!("gold:   {}", gold);
}

