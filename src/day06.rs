static INPUT: &'static str = include_str!("input/day06.txt");

pub fn silver_and_gold<const N: usize>() {
    use std::collections::HashSet;

    let mut set: HashSet<u8, _> = HashSet::with_capacity(N);

    for (i, wind) in INPUT.as_bytes().windows(N).enumerate() {
        set.extend(wind.iter().cloned());

        if set.len() == N {
            println!("marker/badge: {}", i + N);
            break;
        }

        set.clear();
    }
}
