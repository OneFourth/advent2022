use std::collections::HashSet;

use util::*;

#[derive(Debug, Copy, Clone)]
enum Direction {
    U,
    R,
    L,
    D,
}

struct Step {
    direction: Direction,
    count: usize,
}

impl Step {
    fn new(input: &str) -> Self {
        let mut s = input.split(' ');
        let dir = s.next().unwrap();
        let count = s.next().unwrap().parse().unwrap();

        match dir {
            "U" => Step {
                direction: Direction::U,
                count,
            },
            "R" => Step {
                direction: Direction::R,
                count,
            },
            "L" => Step {
                direction: Direction::L,
                count,
            },
            "D" => Step {
                direction: Direction::D,
                count,
            },
            _ => panic!("Invalid"),
        }
    }
}

struct Day09 {
    steps: Vec<Step>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn apply(&mut self, d: Direction) {
        match d {
            Direction::U => self.y -= 1,
            Direction::D => self.y += 1,
            Direction::L => self.x -= 1,
            Direction::R => self.x += 1,
        };
    }

    fn follow(&mut self, other: Position) {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        if dx.abs() == 2 || dy.abs() == 2 {
            self.x += dx.signum();
            self.y += dy.signum();
        }
    }
}

impl Day for Day09 {
    fn parse_input(input: &str) -> Self {
        let steps = input.lines().map(Step::new).collect();

        Self { steps }
    }

    fn part1(&self) -> String {
        let mut h = Position::new();
        let mut t = Position::new();

        let mut unique_pos = HashSet::new();

        for s in &self.steps {
            for _ in 0..s.count {
                h.apply(s.direction);
                t.follow(h);
                unique_pos.insert(t);
            }
        }

        unique_pos.len().to_string()
    }

    fn part2(&self) -> String {
        let mut h = Position::new();
        let mut t = [Position::new(); 9];

        let mut unique_pos = HashSet::new();

        for s in &self.steps {
            for _ in 0..s.count {
                h.apply(s.direction);
                t[0].follow(h);
                for i in 1..t.len() {
                    t[i].follow(t[i - 1]);
                }
                unique_pos.insert(*t.last().unwrap());
            }
        }

        unique_pos.len().to_string()
    }

    fn number() -> u8 {
        9
    }
}

fn main() {
    Day09::run();
}
