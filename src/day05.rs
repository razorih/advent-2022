use std::{collections::VecDeque, iter::FromIterator};

static INPUT: &'static str = include_str!("input/day05.txt");

enum State { Building, Moving }

pub fn silver() {
    let mut lines = INPUT.lines().peekable();
    let mut state = State::Building;

    let num_stacks = (lines.peek().unwrap().len() / 4) + 1; // +1 to account for last missing whitespace
    println!("num stacks: {}", num_stacks);
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); num_stacks];

    for line in lines {
        if line.starts_with(&" 1") { // Skip stack number line
            continue;
        }

        if line.is_empty() { // Stack building and moving separator line
            //println!("Moving to procedure part!");
            state = State::Moving;
            continue; // Skip this empty line
        }

        match state {
            State::Building => {
                for (i, chunk) in line.as_bytes()
                    .chunks(4)
                    .map(|buf|
                        // SAFETY: buf comes to from a valid ascii str with no multibyte characters.
                        unsafe { std::str::from_utf8_unchecked(buf) }
                    )
                    .enumerate()
                {
                    let chunk = chunk.trim_matches(|c| c == ' ' || c == '[' || c == ']');
                    if chunk.is_empty() {
                        continue;
                    }

                    stacks[i].push_front(chunk.chars().nth(0).unwrap());            
                }
            },
            State::Moving => {
                let mut parts = line.split_ascii_whitespace();
                // Quick parser
                parts.next(); // Discard "move"
                let count: usize = parts.next().unwrap().parse().unwrap();
                parts.next(); // Discard "from"
                let from = parts.next().unwrap().parse::<usize>().unwrap() - 1;
                parts.next(); // Discard "to"
                let to = parts.next().unwrap().parse::<usize>().unwrap() - 1;

                //println!("{} | {} -> {}", count, from, to);
                for _ in 0..count {
                    let temp = stacks[from].pop_back().unwrap();
                    stacks[to].push_back(temp);
                }
            }
        }
    }

    // Read answer
    let answer = stacks.iter_mut().map(|stack| stack.back().unwrap());
    println!("{:?}", String::from_iter(answer));
}

pub fn gold() {
    let mut lines = INPUT.lines().peekable();
    let mut state = State::Building;

    let num_stacks = (lines.peek().unwrap().len() / 4) + 1; // +1 to account for last missing whitespace
    println!("num stacks: {}", num_stacks);
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); num_stacks];

    for line in lines {
        if line.starts_with(&" 1") { // Skip stack number line
            continue;
        }

        if line.is_empty() { // Stack building and moving separator line
            //println!("Moving to procedure part!");
            state = State::Moving;
            continue; // Skip this empty line
        }

        match state {
            State::Building => {
                for (i, chunk) in line.as_bytes()
                    .chunks(4)
                    .map(|buf|
                        // SAFETY: buf comes to from a valid ascii str with no multibyte characters.
                        unsafe { std::str::from_utf8_unchecked(buf) }
                    )
                    .enumerate()
                {
                    let chunk = chunk.trim_matches(|c| c == ' ' || c == '[' || c == ']');
                    if chunk.is_empty() {
                        continue;
                    }

                    stacks[i].push_front(chunk.chars().nth(0).unwrap());            
                }
            },
            State::Moving => {
                //println!("{:?}", &stacks);
                let mut parts = line.split_ascii_whitespace();
                // Quick parser
                parts.next(); // Discard "move"
                let count: usize = parts.next().unwrap().parse().unwrap();
                parts.next(); // Discard "from"
                let from = parts.next().unwrap().parse::<usize>().unwrap() - 1;
                parts.next(); // Discard "to"
                let to = parts.next().unwrap().parse::<usize>().unwrap() - 1;

                // Pop "count" elements from stack
                let le = stacks[from].len();
                let mut temp = stacks[from].split_off(le - count);
                //println!("moving: {:?}", &temp);
                stacks[to].append(&mut temp);
            }
        }
    }

    // Read answer
    //println!("{:?}", &stacks);
    let answer = stacks.iter_mut().map(|stack| stack.back().unwrap());
    println!("{:?}", String::from_iter(answer));
}
