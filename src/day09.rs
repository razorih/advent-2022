use std::collections::HashSet;

static INPUT: &'static str = include_str!("input/day09.txt");

#[derive(Debug)]
struct Rope<const N: usize> {
    knots: [(i64, i64); N],
}

impl<const N: usize> Default for Rope<N> {
    fn default() -> Self {
        Self {
           knots: std::array::from_fn(|_| (0, 0))
        }
    }
}

impl<const N: usize> Rope<N> {
    /// Move specific knot of the rope to absolute coordinate
    /// Note: Doesn't fix the rope afterwards
    fn move_abs(&mut self, i: usize, c: (i64, i64)) {
        self.knots[i] = (c.0, c.1);
    }

    /// Moves given knot by some delta
    fn move_delta(&mut self, i: usize, c: (i64, i64)) {
        let pos = self.knots[i];
        self.move_abs(i, (pos.0 + c.0, pos.1 + c.1));
    }
}

pub fn silver_and_gold<const N: usize>() {
    let mut rope: Rope<N> = Rope::default();
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    visited.insert((0, 0));

    for line in INPUT.lines() {
        let (dir, steps) = line.split_once(' ').unwrap();
        let steps: i64 = steps.parse().unwrap();
        let dir: (i64, i64) = match dir {
            "R" => ( 1,  0),
            "L" => (-1,  0),
            "U" => ( 0,  1),
            "D" => ( 0, -1),
            _   => panic!(), // just panic lemao
        };


        for _ in 0..steps {
            for i in 0..N-1 { // Loop through each knot
                if i == 0 { rope.move_delta(0, dir) } // Head moves only on first iteration

                // Check if current head has moved too far away from its tail
                let (dx, dy) = ( // How much head moved
                    rope.knots[i].0 - rope.knots[i+1].0,
                    rope.knots[i].1 - rope.knots[i+1].1,
                );
                if std::cmp::max(dx.abs(), dy.abs()) > 1 { // Check if we moved to far (Chebyshev distance)
                    // Sign function is helpful here since we can only move one tile at a time
                    let mx = dx.signum();
                    let my = dy.signum();

                    rope.move_delta(i+1, (mx, my)); // Move tail to catch up
                }
            }

            visited.insert(rope.knots[N-1]);
        }
    }
    println!("Uniq visits: {}", visited.len());
}
