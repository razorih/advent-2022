use itertools::{Itertools, MinMaxResult};
use std::collections::HashSet;

static INPUT: &'static str = include_str!("input/day18.txt");


type Point = (i8, i8, i8);
pub fn silver() {
    // Put all cubes into a set
    let grid: HashSet<Point> = HashSet::from_iter(INPUT.lines().map(|line| {
        line.split(',')
            .map(|c| c.parse().unwrap())
            .collect_tuple()
            .unwrap()
    }));

    let mut total_free = 0;
    // For each cube, count nonexisting neighbours ie. free faces
    for cube in grid.iter() {
        let nearby: [(i8, i8, i8); 6] = [
            (-1,  0,  0),
            ( 1,  0,  0),
            ( 0, -1,  0),
            ( 0,  1,  0),
            ( 0,  0, -1),
            ( 0,  0,  1),
        ];

        let free = nearby.iter().filter(|dir| {
            let p = (cube.0 + dir.0, cube.1 + dir.1, cube.2 + dir.2);

            !grid.contains(&p)
        }).count();
        total_free += free;
    }

    println!("Silver: {total_free}\n");
}

/// Returns `true` if given point is contained withing given bounds.
/// False otherwise
fn in_bounds(p: &Point, bx: &(i8, i8), by: &(i8, i8), bz: &(i8, i8)) -> bool {
    if p.0 < bx.0 || p.0 > bx.1 {
        return false;
    }

    if p.1 < by.0 || p.1 > by.1 {
        return false;
    }

    if p.2 < bz.0 || p.2 > bz.1 {
        return false;
    }

    true
}

pub fn gold() {
    // Put all cubes into a set
    let grid: HashSet<Point> = HashSet::from_iter(INPUT.lines().map(|line| {
        line.split(',')
            .map(|c| c.parse().unwrap())
            .collect_tuple()
            .unwrap()
    }));

    // Calculate boundary
    let b_x = grid.iter().minmax_by_key(|p| p.0);
    let b_y = grid.iter().minmax_by_key(|p| p.1);
    let b_z = grid.iter().minmax_by_key(|p| p.2);

    let bx = match b_x {
        MinMaxResult::MinMax(min, max) => (min.0, max.0),
        _ => unreachable!(),
    };
    let by = match b_y {
        MinMaxResult::MinMax(min, max) => (min.1, max.1),
        _ => unreachable!(),
    };
    let bz = match b_z {
        MinMaxResult::MinMax(min, max) => (min.2, max.2),
        _ => unreachable!(),
    };

    println!("Grid bounds: x={bx:?}, y={by:?}, z={bz:?}");

    let mut total_free = 0;
    // For each cube, count neighbours that don't exist
    for cube in grid.iter() {
        const NEARBY: [(i8, i8, i8); 6] = [
            (-1,  0,  0),
            ( 1,  0,  0),
            ( 0, -1,  0),
            ( 0,  1,  0),
            ( 0,  0, -1),
            ( 0,  0,  1),
        ];


        let free = NEARBY.iter().filter(|dir| {
            let p = (cube.0 + dir.0, cube.1 + dir.1, cube.2 + dir.2);

            if grid.contains(&p) { // Neighbour blocked, not visible
                return false;
            }

            // Encountered air!
            //
            // Do a depth first search to see if we can reach boundary from this position
            // If we can, ie. at some point we encounter a point that is NOT in bounds
            // then, we count this face as visible.
            let mut s = Vec::new();
            let mut seen: HashSet<Point> = HashSet::new();

            s.push(p);
            while let Some(v) = s.pop() {
                if !in_bounds(&v, &bx, &by, &bz) {
                    // println!("Point {:?} NOT in bounds!", &v);
                    return true;
                }

                if !seen.contains(&v) {
                    seen.insert(v);
                    // Push reachable neighbours
                    s.extend(NEARBY.iter().filter_map(|d| {
                        let p = (v.0 + d.0, v.1 + d.1, v.2 + d.2);
                        if grid.contains(&p) {
                            None
                        } else {
                            Some(p)
                        }
                    }));
                }
            }

            return false; // Exhausted search space, this is a true pocket 
        }).count();

        total_free += free;
    }

    println!("Gold: {total_free}");
}
