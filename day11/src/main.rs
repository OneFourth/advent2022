use util::*;

struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> usize>,
}

impl Monkey {
    fn new(
        items: Vec<usize>,
        o: Box<dyn Fn(usize) -> usize>,
        t: Box<dyn Fn(usize) -> usize>,
    ) -> Self {
        Self {
            items,
            operation: Box::new(o),
            test: Box::new(t),
        }
    }
}

struct Day11;

fn get_monkeys() -> Vec<Monkey> {
    vec![
        Monkey::new(
            vec![99, 67, 92, 61, 83, 64, 98],
            Box::new(|old: usize| old * 17),
            Box::new(|value: usize| if value % 3 == 0 { 4 } else { 2 }),
        ),
        Monkey::new(
            vec![78, 74, 88, 89, 50],
            Box::new(|old: usize| old * 11),
            Box::new(|value: usize| if value % 5 == 0 { 3 } else { 5 }),
        ),
        Monkey::new(
            vec![98, 91],
            Box::new(|old: usize| old + 4),
            Box::new(|value: usize| if value % 2 == 0 { 6 } else { 4 }),
        ),
        Monkey::new(
            vec![59, 72, 94, 91, 79, 88, 94, 51],
            Box::new(|old: usize| old * old),
            Box::new(|value: usize| if value % 13 == 0 { 0 } else { 5 }),
        ),
        Monkey::new(
            vec![95, 72, 78],
            Box::new(|old: usize| old + 7),
            Box::new(|value: usize| if value % 11 == 0 { 7 } else { 6 }),
        ),
        Monkey::new(
            vec![76],
            Box::new(|old: usize| old + 8),
            Box::new(|value: usize| if value % 17 == 0 { 0 } else { 2 }),
        ),
        Monkey::new(
            vec![69, 60, 53, 89, 71, 88],
            Box::new(|old: usize| old + 5),
            Box::new(|value: usize| if value % 19 == 0 { 7 } else { 1 }),
        ),
        Monkey::new(
            vec![72, 54, 63, 80],
            Box::new(|old: usize| old + 3),
            Box::new(|value: usize| if value % 7 == 0 { 1 } else { 3 }),
        ),
    ]
}

impl Day for Day11 {
    fn parse_input(_input: &str) -> Self {
        Self
    }

    fn part1(&self) -> String {
        let mut monkeys = get_monkeys();
        let mut count = Vec::new();
        count.resize(monkeys.len(), 0);

        for _round in 0..20 {
            for i in 0..monkeys.len() {
                let mut moves = Vec::new();
                for item in &monkeys[i].items {
                    let mut new = (monkeys[i].operation)(*item);
                    count[i] += 1;
                    new = new / 3;

                    let index = (monkeys[i].test)(new);
                    moves.push((index, new));
                }
                monkeys[i].items.clear();

                for (index, new) in moves {
                    monkeys[index].items.push(new);
                }
            }
        }

        count.sort();

        count.iter().rev().take(2).product::<usize>().to_string()
    }

    fn part2(&self) -> String {
        let mut monkeys = get_monkeys();
        let mut count = Vec::new();
        count.resize(monkeys.len(), 0);

        const MAGIC: usize = 9699690;

        for _round in 0..10000 {
            for i in 0..monkeys.len() {
                let mut moves = Vec::new();
                for item in &monkeys[i].items {
                    let mut new = (monkeys[i].operation)(*item);
                    new %= MAGIC;
                    count[i] += 1;

                    let index = (monkeys[i].test)(new);
                    moves.push((index, new));
                }
                monkeys[i].items.clear();

                for (index, new) in moves {
                    monkeys[index].items.push(new);
                }
            }
        }

        count.sort();

        count.iter().rev().take(2).product::<usize>().to_string()
    }

    fn number() -> u8 {
        11
    }
}

fn main() {
    Day11::run();
}
