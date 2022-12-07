static INPUT: &'static str = include_str!("input/day02.txt");

#[derive(PartialEq, Clone, Copy)]
enum Hand {
    Rock, Paper, Scissors
}

enum Outcome {
    Win, Lose, Draw
}

impl Hand {
    fn wins(&self, other: &Hand) -> Outcome {
        if self == other {
            return Outcome::Draw;
        }

        match (self, other) {
            (Hand::Rock, Hand::Scissors) => Outcome::Win,
            (Hand::Scissors, Hand::Paper) => Outcome::Win,
            (Hand::Paper, Hand::Rock) => Outcome::Win,
            _ => Outcome::Lose
        }
    }

    fn score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

impl std::str::FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err("bad input"),
        }
    }
}

impl std::str::FromStr for Outcome {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _   => Err("nope"),
        }
    }
}

pub fn silver() {
    let mut total = 0;

    for line in INPUT.lines() {
        let game = line
            .split_whitespace()
            .map(|it| it.parse::<Hand>().unwrap())
            .collect::<Vec<Hand>>();

        let (you, them) = (game[1], game[0]);

        total += match you.wins(&them) {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        };

        total += you.score();
    }

    println!("Total score: {total}");
}


pub fn gold() {
    let mut total = 0;

    for line in INPUT.lines() {
        let mut line = line.split_whitespace();
        let them: Hand = line.next().map(|it| it.parse().unwrap()).unwrap();
        let outcome: Outcome = line.next().map(|it| it.parse().unwrap()).unwrap();

        total += match outcome {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        };

        // What we need to pick
        let you = match outcome {
            Outcome::Draw => them,
            Outcome::Win => match them { // For us to win
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissors,
                Hand::Scissors => Hand::Rock,
            },
            Outcome::Lose => match them { // For us to lose
                Hand::Rock => Hand::Scissors,
                Hand::Paper => Hand::Rock,
                Hand::Scissors => Hand::Paper,
            }
        };

        total += you.score();
    }

    println!("Total score: {total}");
}
