use std::{str::FromStr, convert::Infallible, ops::RangeInclusive, fmt::Debug};
use itertools::Itertools;

static INPUT: &'static str = include_str!("input/day15.txt");

type Point = (isize, isize);
#[derive(Debug, Clone, Copy)]
struct Sensor {
    position: Point,
    beacon_position: Point,
    dist: isize, // Manhattan distance from pos to beacon_pos
}

/// Calculate Manhattan distance between two points
fn manhattan(a: &Point, b: &Point) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

impl FromStr for Sensor {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        /// Parse coordinate in format "x=0, y=0"
        fn parse_coord(s: &str) -> Point {
            let (sx, sy) = s.split_once(", ").unwrap();
            let (_, sx) = sx.split_once('=').unwrap();
            let (_, sy) = sy.split_once('=').unwrap();

            (sx.parse().unwrap(), sy.parse().unwrap())
        }

        let (sensor, beacon) = s.split_once(": ").unwrap();
        let sensor = sensor.strip_prefix("Sensor at ").unwrap();
        let beacon = beacon.strip_prefix("closest beacon is at ").unwrap();

        let sensor = parse_coord(sensor);
        let beacon = parse_coord(beacon);

        Ok(Self {
            position: sensor,
            beacon_position: beacon,
            dist: manhattan(&sensor, &beacon),
        })
    }
}

fn merge_ranges<T: Copy + Ord>(ranges: &[RangeInclusive<T>]) -> Vec<RangeInclusive<T>> {
    // First, sort by starting position
    let mut sorted = ranges.to_vec();
    let mut result = Vec::new();
    sorted.sort_unstable_by_key(|r| *r.start());

    result.push(sorted[0].clone());
    for i in 1..sorted.len() {
        let latest = result.last_mut().unwrap();
        let next = &sorted[i];

        if latest.end() >= next.start() {
            *latest = (*latest.start())..=(std::cmp::max(*next.end(), *latest.end()));
        } else {
            result.push(next.clone());
        }
    }

    result
}

pub fn silver() {
    let sensors: Vec<Sensor> = INPUT.lines().map(|s| s.parse().unwrap()).collect();
    const Y: isize = 2_000_000;

    // Check which sensors' range can reach to Y
    // ie. are relevant
    let reachable: Vec<_> = sensors.into_iter()
        .filter(|sensor| (sensor.position.1 - Y).abs() <= sensor.dist)
        .collect();

    // Count the number of existing unique beacons on Y
    let count = reachable.iter()
        .filter(|s| s.beacon_position.1 == Y)
        .unique_by(|s| s.beacon_position)
        .count() as isize;
    println!("number of uniq beacons on Y: {count}");

    // Calculate coverage range for each sensor on level Y
    let ranges: Vec<RangeInclusive<_>> = reachable.iter().map(|s| {
        let to_y = (s.position.1 - Y).abs(); // vertical distance from sensor to Y
        let rem = (to_y - s.dist).abs(); // Remaining distance from Y to triangle apex

        // Any horizontally sliced isosceles triangle is still a isosceles triangle
        // This range represents its base
        (s.position.0 - rem)..=(s.position.0 + rem)
    }).collect();

    let merged = merge_ranges(&ranges);
    println!("merged ranges\n  {ranges:?}\ninto:\n  {merged:?}");

    let silver = merged
        .iter()
        .fold(0,
            // calculate number of covered positions
            |acc, r| acc + ((*r.end() - *r.start()).abs() + 1 as isize)
        ) - count;
    println!("Silver: {silver}\n");
}

pub fn gold() {
    let sensors: Vec<Sensor> = INPUT.lines().map(|s| s.parse().unwrap()).collect();

    for y in 0..=4_000_000 { // loops go brrr
        // Calculate coverage range for each sensor on level Y
        let ranges: Vec<RangeInclusive<_>> = sensors.iter().filter_map(|s| {
            let to_y = (s.position.1 - y).abs(); // vertical distance from sensor to Y

            if to_y > s.dist { // Filter out sensors that cannot reach this Y level
                return None
            }

            let rem = (to_y - s.dist).abs(); // Remaining distance from Y to triangle apex

            // Any horizontally sliced isosceles triangle is still a isosceles triangle
            // This range represents its base
            Some((s.position.0 - rem)..=(s.position.0 + rem))
        }).collect();

        let merged = merge_ranges(&ranges);
        // Look for positions where ranges couldn't be merged into one
        // Missing beacon is located there
        if merged.len() > 1 {
            println!("!!! found a hole!!! {:?}", merged);
            let x = *merged[0].end() + 1;
            println!("Gold: x={x}, y={y}, ans={}", x*4_000_000 + y);
            break;
        }
    }
}
