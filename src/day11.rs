use std::collections::VecDeque;

type Worry = f64;
struct Monkey {
    /// Item worry levels
    items: VecDeque<Worry>,
    /// Inspection function pointer, takes old worry level and returns a new one
    op: Box<dyn Fn(Worry) -> Worry>,
    /// Tuple containing divisor, and target monkeys for true and false paths
    test: (Worry, usize, usize),
    /// How many times has this monkey inspected an item
    inspections: usize,
}

impl Monkey {
    fn new(
        items: Vec<Worry>,
        op: impl Fn(Worry) -> Worry + 'static,
        test: (Worry, usize, usize)
    ) -> Self {
        Self {
            items: items.into(),
            op: Box::new(op),
            test,
            inspections: 0
        }
    }
}

pub fn silver_and_gold<const N: usize, const S: bool>() {
    let _example = [
        Monkey::new(vec![79., 98.],           |old| old * 19., (23., 2, 3)),
        Monkey::new(vec![54., 65., 75., 74.], |old| old + 6.,  (19., 2, 0)),
        Monkey::new(vec![79., 60., 97.],      |old| old * old, (13., 1, 3)),
        Monkey::new(vec![74.],                |old| old + 3.,  (17., 0, 1)),
    ];

    let _real = [
        Monkey::new(vec![56., 52., 58., 96., 70., 75., 72.],      |old| old * 17., (11., 2, 3)),
        Monkey::new(vec![75., 58., 86., 80., 55., 81.],           |old| old + 7.,  (3.,  6, 5)),
        Monkey::new(vec![73., 68., 73., 90.],                     |old| old * old, (5.,  1, 7)),
        Monkey::new(vec![72., 89., 55., 51., 59.],                |old| old + 1.,  (7.,  2, 7)),
        Monkey::new(vec![76., 76., 91.],                          |old| old * 3.,  (19., 0, 3)),
        Monkey::new(vec![88.],                                    |old| old + 4.,  (2.,  6, 4)),
        Monkey::new(vec![64., 63., 56., 50., 77., 55., 55., 86.], |old| old + 8.,  (13., 4, 0)),
        Monkey::new(vec![79., 58.],                               |old| old + 6.,  (17., 1, 5)),
    ];

    let mut input = _real;
    // Product of all test divisors
    let prod = input.iter().fold(1., |acc, m| acc * m.test.0);

    for _round in 0..N  {
        for monkey_i in 0..input.len() {
            while let Some(item) = input[monkey_i].items.pop_front() {
                input[monkey_i].inspections += 1;

                // Calculate new worry level
                let item = if S { // Solving silver
                    ((input[monkey_i].op)(item) / 3.0).floor()
                } else { // Solving gold
                    (input[monkey_i].op)(item) % prod
                };

                // Which monkey should item be thrown at
                let target_i = if item % input[monkey_i].test.0 == 0.0 {
                    input[monkey_i].test.1
                } else {
                    input[monkey_i].test.2
                };

                //println!("Throwing {item} from {monkey_i} to {target_i}");
                input[target_i].items.push_back(item);
            }
        }
    }

    for monke in &input {
        println!("Inventory: {:?} | count: {}", monke.items, monke.inspections);
    }

    // Monkey business
    input.sort_unstable_by(|a, b| b.inspections.cmp(&a.inspections));
    let monkey_business = input[0].inspections * input[1].inspections;

    println!("Monkey business: {monkey_business}");
}
