static INPUT: &'static str = include_str!("input/day01.txt");

pub fn silver() {
    let mut current_sum = 0;
    let mut max_sum = 0;

    for line in INPUT.lines() {
        if line.is_empty() {
            // Reset group
            if current_sum > max_sum {
                max_sum = current_sum;
            }

            current_sum = 0;
            continue;
        } else if let Ok(calories) = line.parse::<i32>() {
            current_sum += calories;
        } else {
            panic!("bad input");
        }
    }

    println!("Most calories: {}", max_sum);
}


pub fn gold() {
    use std::collections::BinaryHeap;

    let mut current_sum = 0;
    let mut all_calories: BinaryHeap<i32> = BinaryHeap::new();

    for line in INPUT.lines() {
        if line.is_empty() {
            // Whitespace, push sum
            all_calories.push(current_sum);
            current_sum = 0;
            continue;
        } else if let Ok(calories) = line.parse::<i32>() {
            current_sum += calories;
        } else {
            panic!("bad input")
        }
    }

    let mut final_sum = 0;
    for _ in 0..3 {
        final_sum += all_calories.pop().unwrap();
    }

    println!("Final sum: {}", final_sum)
}
