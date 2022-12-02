use util::*;

#[derive(Debug, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn choose(c: char) -> Shape {
        match c {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("Invalid"),
        }
    }

    fn outcome(theirs: Shape, c: char) -> Shape {
        match (theirs, c) {
            (Shape::Rock, 'X') => Shape::Scissors,
            (Shape::Paper, 'X') => Shape::Rock,
            (Shape::Scissors, 'X') => Shape::Paper,
            (s, 'Y') => s,
            (Shape::Rock, 'Z') => Shape::Paper,
            (Shape::Paper, 'Z') => Shape::Scissors,
            (Shape::Scissors, 'Z') => Shape::Rock,
            _ => panic!("Invalid"),
        }
    }

    fn score(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[derive(Debug)]
struct Round {
    theirs: Shape,
    mine: Shape,
}

impl Round {
    fn score(&self) -> usize {
        let outcome = match (&self.theirs, &self.mine) {
            (Shape::Rock, Shape::Rock) => 3,
            (Shape::Rock, Shape::Paper) => 6,
            (Shape::Rock, Shape::Scissors) => 0,
            (Shape::Paper, Shape::Rock) => 0,
            (Shape::Paper, Shape::Paper) => 3,
            (Shape::Paper, Shape::Scissors) => 6,
            (Shape::Scissors, Shape::Rock) => 6,
            (Shape::Scissors, Shape::Paper) => 0,
            (Shape::Scissors, Shape::Scissors) => 3,
        };

        outcome + self.mine.score()
    }
}

#[derive(Debug)]
struct Day02 {
    part1_rounds: Vec<Round>,
    part2_rounds: Vec<Round>,
}

impl Day for Day02 {
    fn parse_input(input: &str) -> Self {
        let part1_rounds = input
            .lines()
            .map(|s| Round {
                theirs: Shape::choose(s.chars().next().expect("No char")),
                mine: Shape::choose(s.chars().nth(2).expect("No char")),
            })
            .collect();
        let part2_rounds = input
            .lines()
            .map(|s| {
                let theirs = Shape::choose(s.chars().next().expect("No char"));
                Round {
                    theirs,
                    mine: Shape::outcome(theirs, s.chars().nth(2).expect("No char")),
                }
            })
            .collect();

        Self {
            part1_rounds,
            part2_rounds,
        }
    }

    fn part1(&self) -> String {
        self.part1_rounds
            .iter()
            .map(Round::score)
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.part2_rounds
            .iter()
            .map(Round::score)
            .sum::<usize>()
            .to_string()
    }

    fn number() -> u8 {
        2
    }
}

fn main() {
    Day02::run();
}
