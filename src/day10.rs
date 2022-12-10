use std::convert::Infallible;

static INPUT: &'static str = include_str!("input/day10.txt");

#[derive(Debug)]
struct Machine {
    cycle: usize,
    x: i64,
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            cycle: 1,
            x: 1,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i64)
}

impl std::str::FromStr for Instruction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some((_inst, arg)) => {
                // Assume that instruction is always addx
                Ok(Instruction::Addx(arg.parse().unwrap()))
            },
            None => Ok(Instruction::Noop),
        }
    }
}

impl Machine {
    fn write_if_interesting(&self, out: &mut Vec<i64>) {
        const POINTS: [i64; 6] = [20, 60, 100, 140, 180, 220];
        // POINTS array is sorted so binary search can be used.
        if let Ok(index) = POINTS.binary_search(&(self.cycle as i64)) {
            out.push(POINTS[index] * self.x);
        }
    }

    fn draw(&self) {
        let h_pos = (self.cycle - 1) % 40;

        if h_pos == 0 {
            print!("\n");
        }
        if (h_pos as i64 - self.x).abs() <= 1 {
            print!("█"); // Actual real "pixels"
        } else {
            print!(" ");
        }
    }
}

pub fn silver_and_gold() {
    let mut machine = Machine::default();
    let mut strengths: Vec<i64> = Vec::new();

    print!("█"); // getting started :^)
    for line in INPUT.lines() {
        let inst: Instruction = line.parse().unwrap();

        machine.cycle += 1;
        machine.draw();
        match inst {
            Instruction::Noop => {},
            Instruction::Addx(val) => {
                // Do nothing
                machine.write_if_interesting(&mut strengths);

                // Take extra cycle to increment X
                machine.cycle += 1;
                machine.x += val;
                machine.draw();
            }
        }
        machine.write_if_interesting(&mut strengths); // Combined check for cycles ending NOOP and ADDX
    }

    println!("\nSilver: {:?}", strengths.iter().sum::<i64>());
}
