use std::ops::RangeInclusive;

use util::*;

struct Assignment {
    first: RangeInclusive<usize>,
    second: RangeInclusive<usize>,
}

impl Assignment {
    fn overlaps_completely(&self) -> bool {
        self.second.clone().all(|v| self.first.contains(&v))
            || self.first.clone().all(|v| self.second.contains(&v))
    }

    fn overlaps_any(&self) -> bool {
        self.second.clone().any(|v| self.first.contains(&v))
            || self.first.clone().any(|v| self.second.contains(&v))
    }
}

struct Day04 {
    assignments: Vec<Assignment>,
}

impl Day for Day04 {
    fn parse_input(input: &str) -> Self {
        let r = regex::Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
        let assignments = input
            .lines()
            .map(|s| {
                let c = r
                    .captures(s)
                    .unwrap()
                    .iter()
                    .skip(1)
                    .map(|s| s.unwrap().as_str().parse().unwrap())
                    .collect::<Vec<_>>();
                Assignment {
                    first: c[0]..=c[1],
                    second: c[2]..=c[3],
                }
            })
            .collect();

        Day04 { assignments }
    }

    fn part1(&self) -> String {
        self.assignments
            .iter()
            .filter(|a| a.overlaps_completely())
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        self.assignments
            .iter()
            .filter(|a| a.overlaps_any())
            .count()
            .to_string()
    }

    fn number() -> u8 {
        4
    }
}

fn main() {
    Day04::run();
}
