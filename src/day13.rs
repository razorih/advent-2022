use miniserde::json::{self, Value, Number};
use itertools::Itertools;

static INPUT: &'static str = include_str!("input/day13.txt");

/// Recursively compare two JSON values.
///
/// Returns [`Some(bool)`] if order could be determined.
/// [`None`] otherwise.
fn compare_recursive(left: &json::Value, right: &json::Value) -> Option<bool> {
    match (left, right) {
        // Base case for two numbers
        (Value::Number(Number::U64(l)), Value::Number(Number::U64(r))) => {
            if l < r {
                return Some(true);
            } else if l > r {
                return Some(false);
            } else {
                None
            }
        },
        // Recursing case for nested arrays
        (Value::Array(l), Value::Array(r)) => {
            // Take value from both arrays and retry comparison until either:
            // 1. Order could be determined (base case above)
            // 2. End of either list is reached.
            //    Base case is then determined by length of arrays (below)
            for (a, b) in l.iter().zip(r) {
                let res = compare_recursive(a, b); // Recursion beibeee
                if res.is_some() { return res; }
            }

            // Returning from nested arrays
            // Check which array is "empty"
            if l.len() > r.len() {
                Some(false)
            } else if l.len() < r.len() {
                Some(true)
            } else {
                None
            }
        },
        // Array and number cases, wrap number into array and retry
        (l, Value::Number(r)) => {
            let mut newright = json::Array::new();
            newright.push(Value::Number(r.clone()));

            compare_recursive(l, &Value::Array(newright))
        },
        (Value::Number(l), r) => {
            let mut newleft = json::Array::new();
            newleft.push(Value::Number(l.clone()));

            compare_recursive(&Value::Array(newleft), r)
        },
        _ => unreachable!(),
    }
}

pub fn silver() {
    let mut count: usize = 0;
    let mut pair_i: usize = 0;

    for mut line in &INPUT.lines().chunks(3) {
        pair_i += 1;
        if let (Some(left), Some(right), _) = (line.next(), line.next(), line.next()) {
            let left: json::Value  = json::from_str(left).unwrap();
            let right: json::Value = json::from_str(right).unwrap();
            
            let is_in_order = compare_recursive(&left, &right);

            if let Some(true) = is_in_order {
                // println!("Pair {pair_i} is in order");
                count += pair_i;
            }
        }
    }

    println!("Silver: {} (pairs: {})", count, pair_i);
}

pub fn gold() {
    // Collect all lines into a vec
    let mut input: Vec<&str> = INPUT.lines()
        .chain(["", "[[2]]", "[[6]]"])
        .filter(|line| !line.is_empty())
        .collect();

    input.sort_unstable_by(|&left, &right| {
        let left: json::Value  = json::from_str(left).unwrap();
        let right: json::Value = json::from_str(right).unwrap();

        use std::cmp::Ordering;
        match compare_recursive(&left, &right) { // Ascending sort
            Some(false) => Ordering::Greater,
            Some(true)  => Ordering::Less,
            _           => Ordering::Equal,
        }
    });

    // Find decoder key indices from sorted input array
    let first = input.iter().position(|&s| s == "[[2]]").unwrap() + 1;
    let second = input.iter().position(|&s| s == "[[6]]").unwrap() + 1;

    println!("Gold: {}", first * second);
}
