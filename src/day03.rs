static INPUT: &'static str = include_str!("input/day03.txt");

fn priority(i: u8) -> u8 {
    match i {
        b'A'..=b'Z' => i - (b'A' - 1) + 26,
        b'a'..=b'z' => i - (b'a' - 1),
        _ => panic!("bad priority character"),
    }
}

pub fn silver() {
    use std::collections::BTreeSet;

    let mut first_set: BTreeSet<u8> = BTreeSet::new();
    let mut second_set: BTreeSet<u8> = BTreeSet::new();
    let mut total_sum: u64 = 0;

    for line in INPUT.lines() {
        let len = line.len();

        line[..(len / 2)].as_bytes().iter().for_each(|c| {first_set.insert(*c);});
        line[(len / 2)..].as_bytes().iter().for_each(|c| {second_set.insert(*c);});

        let intersection = first_set
            .intersection(&second_set)
            .map(|c| *c as char)
            .inspect(|c| print!("{c} "))
            .collect::<Vec<char>>();
        total_sum += intersection.iter().fold(0u64, |acc, c| acc + priority(*c as u8) as u64);

        first_set.clear();
        second_set.clear();
    }

    println!("\nTotal: {}", total_sum);
}

pub fn gold<const N: usize>() {
    use std::collections::BTreeSet;

    let mut sets: [BTreeSet<u8>; N] = std::array::from_fn(|_| BTreeSet::new());
    let mut total_sum: u64 = 0;

    for chunk in INPUT.lines().collect::<Vec<&str>>().chunks(N) {
        for (i, line) in chunk.iter().enumerate() {
            // Put each line into their own Set
            line.as_bytes().iter().for_each(|c| {sets[i].insert(*c);});
        }

        // Calculate intersection withing all sets eg.
        // ((A . B) . C) . D
        let badge = sets
            .iter()
            .cloned()
            .reduce(|acc, set| &acc & &set)
            .unwrap();

        let prio = priority(*badge.iter().next().unwrap());
        println!("{:?}", prio);

        total_sum += prio as u64;

        // Clear all sets for next chunk
        for set in sets.iter_mut() { set.clear() };
    }

    println!("\nPart 2 Total: {}", total_sum);
}
