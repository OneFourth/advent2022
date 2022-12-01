use util::*;

#[derive(Debug)]
struct Elf {
    food: Vec<usize>,
}

impl Elf {
    fn get_total(&self) -> usize {
        self.food.iter().sum()
    }
}

#[derive(Debug)]
struct Day01 {
    elves: Vec<Elf>,
}

impl Day for Day01 {
    fn parse_input(input: &str) -> Self {
        let elves = input
            .split("\n\n")
            .map(|e| Elf {
                food: e.lines().map(|f| f.parse().expect("Not food")).collect(),
            })
            .collect();

        Day01 { elves }
    }

    fn part1(&self) -> String {
        self.elves
            .iter()
            .map(Elf::get_total)
            .max()
            .expect("No elf")
            .to_string()
    }

    fn part2(&self) -> String {
        let mut sorted: Vec<_> = self.elves.iter().map(Elf::get_total).collect();
        sorted.sort();

        sorted.iter().rev().take(3).sum::<usize>().to_string()
    }

    fn number() -> u8 {
        1
    }
}

fn main() {
    Day01::run();
}
