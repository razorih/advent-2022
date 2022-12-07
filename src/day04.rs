static INPUT: &'static str = include_str!("input/day04.txt");
use std::ops::Range;

trait Overlapping {
    fn is_disjoint(&self, other: &Self) -> bool;
    fn overlaps_fully(&self, other: &Self) -> bool;
}

impl<Idx: PartialOrd<Idx>> Overlapping for Range<Idx> {
    fn is_disjoint(&self, other: &Self) -> bool {
        if self.end < other.start {
            true // Self before other
        } else if other.end < self.start {
            true // Self after other
        } else {
            false
        }
    }

    fn overlaps_fully(&self, other: &Self) -> bool {
        if other.start >= self.start && other.end <= self.end {
            true
        } else if self.start >= other.start && self.end <= other.end {
            true
        } else {
            false
        }
    }
}

fn into_range(s: &str) -> Range<i32> {
    let parts = s.split_once('-').unwrap();

    Range {
        start: parts.0.parse().unwrap(),
        end: parts.1.parse().unwrap(),
    }
}

pub fn silver() {
    let mut count = 0;

    for line in INPUT.lines() {
        let pair = line.split_once(',').unwrap();
        let pair: (Range<i32>, Range<i32>) = (into_range(pair.0), into_range(pair.1));

        if pair.0.overlaps_fully(&pair.1) {
            count += 1;
        }
    }

    println!("Fully contained count: {count}");
}

pub fn gold() {
    let mut count = 0;

    for line in INPUT.lines() {
        let pair = line.split_once(',').unwrap();
        let pair: (Range<i32>, Range<i32>) = (into_range(pair.0), into_range(pair.1));

        if !pair.0.is_disjoint(&pair.1) {
            count += 1;
        }
    }

    println!("Overlaps at all count: {count}");
}
