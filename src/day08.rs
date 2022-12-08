use std::{str::FromStr, convert::Infallible};
use itertools::Itertools;

static INPUT: &'static str = include_str!("input/day08.txt");

#[derive(Debug, Clone)]
struct Grid {
    size: usize,
    trees: Vec<(u8, bool)>,
}

impl Grid {
    fn n(&self, col: usize, row: usize) -> u8 {
        self.trees[row*self.size + col].0
    }

    fn mark(&mut self, col: usize, row: usize) {
        self.trees[row*self.size + col].1 = true;
    }
}

impl FromStr for Grid {
    type Err = Infallible; // just panic lol
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.lines().nth(0).unwrap().len(); // Assume NxN grid
        Ok(Grid {
            size: len,
            trees: Vec::from_iter(
                s.as_bytes()
                .into_iter()
                .filter(|c| c.is_ascii_digit()) // Get rid of newlines
                .map(|&c| (c - b'0', false)) // Convert ASCII number char to actual number quickly
            )
        })
    }
}

pub fn silver() {
    let mut g: Grid = INPUT.parse().unwrap();

    // Columnwise north and south
    for col in 0..g.size {
        // NORTH
        let mut highest = g.n(col, 0); // First tree in current column is always highest
        g.mark(col, 0); // Also always visible
        for i in 1..g.size-1 {
            // Look at next tree,
            let now = g.n(col, i);
            // it is visible if now > highest
            if now > highest {
                highest = now;
                g.mark(col, i); // mark tree as visible

                if now == 9 {
                    break; // There cannot be anything higher than 9
                }
            }
        }

        // SOUTH
        let mut highest = g.n(col, g.size-1); // First tree in current column is always highest
        g.mark(col, g.size-1); // Also always visible
        for i in (1..g.size-1).rev() {
            // Look at next tree,
            let now = g.n(col, i);
            // it is visible if now > highest
            if now > highest {
                highest = now;
                g.mark(col, i); // mark tree as visible

                if now == 9 {
                    break; // There cannot be anything higher than 9
                }
            }
        }
    }

    // Rowwise west and east
    for row in 0..g.size {
        // WEST
        let mut highest = g.n(0, row); // First tree in current column is always highest
        g.mark(0, row); // Also always visible
        for i in 1..g.size-1 {
            // Look at next tree,
            let now = g.n(i, row);
            // it is visible if now > highest
            if now > highest {
                highest = now;
                g.mark(i, row); // mark tree as visible

                if now == 9 {
                    break; // There cannot be anything higher than 9
                }
            }
        }

        // EAST
        let mut highest = g.n(g.size-1, row); // First tree in current column is always highest
        g.mark(g.size-1, row); // Also always visible
        for i in (1..g.size-1).rev() {
            // Look at next tree,
            let now = g.n(i, row);
            // it is visible if now > highest
            if now > highest {
                highest = now;
                g.mark(i, row); // mark tree as visible

                if now == 9 {
                    break; // There cannot be anything higher than 9
                }
            }
        }
    }

    let visible_count = g.trees.iter()
        .filter(|it| it.1)
        .count();
    println!("Visible: {}", visible_count);
}

pub fn gold() {
    let g: Grid = INPUT.parse().unwrap();
    let mut max: usize = 0;

    for col in 1..g.size-1 { // Don't consider edges, their score is always 0
        for row in 1..g.size-1 {
            let now = g.n(col, row);

            // sad: https://github.com/rust-lang/rust/issues/62208
            let mut iter_north = (0..row).rev().map(|i| g.n(col, i)).peekable();
            let mut s_n = iter_north
                .peeking_take_while(|&h| h < now)
                .collect::<Vec<_>>();
            s_n.extend(iter_north.next());

            let mut iter_south = (row+1..g.size).map(|i| g.n(col, i)).peekable();
            let mut s_s = iter_south
                .peeking_take_while(|&h| h < now)
                .collect::<Vec<_>>();
            s_s.extend(iter_south.next());

            let mut iter_east = (0..col).rev().map(|i| g.n(i, row)).peekable();
            let mut s_e = iter_east
                .peeking_take_while(|&h| h < now)
                .collect::<Vec<_>>();
            s_e.extend(iter_east.next());

            let mut iter_west = (col+1..g.size).map(|i| g.n(i, row)).peekable();
            let mut s_w = iter_west
                .peeking_take_while(|&h| h < now)
                .collect::<Vec<_>>();
            s_w.extend(iter_west.next());

            let total = [s_n, s_w, s_e, s_s].iter().map(|v| v.len()).reduce(|acc, it| acc * it).unwrap();
            if total > max {
                max = total;
            }
        }
    }

    println!("Max score: {max}");
}
