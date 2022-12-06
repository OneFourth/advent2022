use util::*;

struct Day06 {
    data: Vec<char>,
}

impl Day06 {
    fn find_unique_window(&self, size: usize) -> usize {
        self.data
            .windows(size)
            .enumerate()
            .find(|&(_, v)| (0..size).all(|i| v[(i + 1)..].iter().all(|&c| v[i] != c)))
            .unwrap()
            .0
            + size
    }
}

impl Day for Day06 {
    fn parse_input(input: &str) -> Self {
        Self {
            data: input.chars().collect(),
        }
    }

    fn part1(&self) -> String {
        self.find_unique_window(4).to_string()
    }

    fn part2(&self) -> String {
        self.find_unique_window(14).to_string()
    }

    fn number() -> u8 {
        6
    }
}

fn main() {
    Day06::run();
}
