static INPUT: &'static str = include_str!("input/day14.txt");
use std::str::FromStr;

use itertools::{Itertools, MinMaxResult};

/// Same as a..=b but supports ranges where a > b
fn generic_range_inclusive(a: usize, b: usize) -> impl Iterator<Item = usize> {
    // Needs boxing because of different return types in branches :(
    let x: Box<dyn Iterator<Item = usize>>;
    x = if a < b {
        Box::new(a..=b)
    } else {
        Box::new((b..=a).rev())
    };

    x
}

#[derive(Debug)]
enum Floor {
    Abyss(usize),
    Infinite(usize),
}

type RockSet<T> = std::collections::HashSet<T>;

#[derive(Debug)]
struct Cave {
    rocks: RockSet<(usize, usize)>,
    source: (usize, usize),
    bounds: (usize, usize), // minimum and maximum x-coordinate where rocks appear
    floor: Floor, // y-coordinate of the infinite floor, None if bottomless
}

impl Cave {
    fn new(rocks: RockSet<(usize, usize)>, source: (usize, usize)) -> Self {
        let bounds = rocks.iter().minmax_by_key(|&i| i.0); // Look for min and max x coordinate
        let bounds = match bounds {
            MinMaxResult::MinMax(min, max) => (min.0, max.0), // Just the x coord
            _ => unreachable!(), // Some silly behaviour going on
        };
        let floor = *rocks.iter().max_by_key(|&&i| i.1).unwrap();

        Self {
            rocks,
            source,
            bounds,
            floor: Floor::Abyss(floor.1),
        }
    }
}

impl FromStr for Cave {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rocks = RockSet::new();

        for path in s.lines() {
            for (start, end) in path.split(" -> ")
                .map(|coord| {
                    let (x, y) = coord.split_once(',').unwrap();
                    (x.parse().unwrap(), y.parse().unwrap()) as (usize, usize)
                })
                .tuple_windows()
            {
                // One of these loops iterates only once
                for x in generic_range_inclusive(start.0, end.0) {
                    for y in generic_range_inclusive(start.1, end.1) {
                        rocks.insert((x, y));
                    }
                }
            }
        }

        Ok(Cave::new(rocks, (500, 0)))
    }
}

/// An unit of sand
struct Sand((usize, usize));

impl Sand {
    /// Simulates time step.
    /// Returns `true` if sand moved somewhere,
    /// `false` if sand didn't move and is resting.
    fn simulate(&mut self, cave: &Cave) -> bool {
        let pos = &mut self.0;
        let rocks = &cave.rocks;

        // Order: Down, LeftDown, RightDown
        // If any succeeds, adjust pos and return true
        // If all fail, don't adjust pos and return false, sand is now resting

        // Handle infinite floor
        if let Floor::Infinite(floor) = cave.floor {
            if pos.1 + 1 >= floor {
                return false; // We cannot move through the floor in any case, sand is resting
            }
        }

        // Down
        if !rocks.contains(&(pos.0, pos.1 + 1))  {
            *pos = (pos.0, pos.1 + 1);
            return true
        }

        // LeftDown
        if !rocks.contains(&(pos.0 - 1, pos.1 + 1)) {
            *pos = (pos.0 - 1, pos.1 + 1);
            return true
        }

        // RightDown
        if !rocks.contains(&(pos.0 + 1, pos.1 + 1)) {
            *pos = (pos.0 + 1, pos.1 + 1);
            return true
        }

        false
    }

    fn take(self) -> (usize, usize) { self.0 }
}

pub fn silver() {
    let mut cave: Cave = INPUT.parse().unwrap();
    let mut steps: usize = 0;

    println!("Cave bounds: {:?}, floor: {:?}", cave.bounds, cave.floor);

    'outer: loop {
        let mut sand = Sand(cave.source); // Create a new piece of sand

        while sand.simulate(&cave) { // Simulate until resting
            if let Floor::Abyss(height) = cave.floor {
                if sand.0.1 > height {
                    println!("Falling into Abyss at ({}, {})", sand.0.0, sand.0.1);
                    break 'outer;
                }
            }
        }

        steps += 1;
        cave.rocks.insert(sand.take());
    }

    println!("Silver: {steps} steps\n");
}

pub fn gold() {
    let mut cave: Cave = INPUT.parse().unwrap();
    // Replace Abyss with Infinite floor two units below
    if let Floor::Abyss(height) = cave.floor {
        cave.floor = Floor::Infinite(height + 2);
    }
    let mut steps: usize = 0;

    println!("Cave bounds: {:?}, floor: {:?}", cave.bounds, cave.floor);

    'outer: loop {
        let mut sand = Sand(cave.source); // Create a new piece of sand

        while sand.simulate(&cave) {} // Simulate without abyss

        // Sand is now resting
        steps += 1;

        let rest = sand.take();
        if rest.0 == cave.source.0 && rest.1 == cave.source.1 {
            println!("Reached source");
            break 'outer;
        }
        cave.rocks.insert(rest);
    }

    println!("Gold: {steps} steps");
}
