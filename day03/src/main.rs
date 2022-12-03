use std::collections::HashSet;

use util::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Item {
    Lower(char),
    Upper(char),
}

impl Item {
    fn priority(&self) -> usize {
        match self {
            Item::Lower(l) => 1 + (*l as usize - 'a' as usize),
            Item::Upper(u) => 27 + (*u as usize - 'A' as usize),
        }
    }
}

#[derive(Debug)]
struct Rucksack {
    items: Vec<Item>,
}

impl Rucksack {
    fn item_type(&self) -> Item {
        let (left, right) = self.items.split_at(self.items.len() / 2);
        let left: HashSet<_> = left.iter().copied().collect();
        let right: HashSet<_> = right.iter().copied().collect();

        *left
            .intersection(&right)
            .next()
            .expect("Could not find match")
    }
}

#[derive(Debug)]
struct Day03 {
    bags: Vec<Rucksack>,
}

impl Day for Day03 {
    fn parse_input(input: &str) -> Self {
        let bags = input
            .lines()
            .map(|s| Rucksack {
                items: s
                    .chars()
                    .map(|c| {
                        if c.is_lowercase() {
                            Item::Lower(c)
                        } else {
                            Item::Upper(c)
                        }
                    })
                    .collect(),
            })
            .collect();

        Self { bags }
    }

    fn part1(&self) -> String {
        self.bags
            .iter()
            .map(|b| b.item_type().priority())
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.bags
            .chunks(3)
            .map(|v| {
                let [a, b, c] = v else { panic!("Invalid chunk") };

                let a: HashSet<_> = a.items.iter().copied().collect();
                let b: HashSet<_> = b.items.iter().copied().collect();
                let c: HashSet<_> = c.items.iter().copied().collect();

                a.intersection(&b)
                    .copied()
                    .collect::<HashSet<_>>()
                    .intersection(&c)
                    .next()
                    .expect("Could not find item")
                    .priority()
            })
            .sum::<usize>()
            .to_string()
    }

    fn number() -> u8 {
        3
    }
}

fn main() {
    Day03::run();
}
