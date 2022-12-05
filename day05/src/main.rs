use util::*;

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_layout(s: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<_>> = Vec::new();

    for l in s.lines() {
        for (i, c) in l.match_indices(|c: char| c.is_ascii_alphabetic()) {
            let index = (i - 1) / 4;

            if index >= stacks.len() {
                stacks.resize(index + 1, Vec::new());
            }

            stacks[index].push(c.chars().next().unwrap());
        }
    }

    for s in &mut stacks {
        s.reverse();
    }

    stacks
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    let r = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    s.lines()
        .map(|l| {
            let c: Vec<_> = r
                .captures(l)
                .unwrap()
                .iter()
                .skip(1)
                .map(|v| v.unwrap().as_str().parse().unwrap())
                .collect();

            Instruction {
                count: c[0],
                from: c[1],
                to: c[2],
            }
        })
        .collect()
}

struct Day05 {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

impl Day for Day05 {
    fn parse_input(input: &str) -> Self {
        let [layout, instructions]: [&str; 2] = input
            .split("\n\n")
            .collect::<Vec<_>>()
            .try_into()
            .expect("Could not split");

        let stacks = parse_layout(layout);
        let instructions = parse_instructions(instructions);

        Self {
            stacks,
            instructions,
        }
    }

    fn part1(&self) -> String {
        let mut state = self.stacks.clone();

        for i in &self.instructions {
            for _ in 0..i.count {
                if let Some(c) = state[i.from - 1].pop() {
                    state[i.to - 1].push(c);
                }
            }
        }

        state.iter().filter_map(|v| v.last()).collect()
    }

    fn part2(&self) -> String {
        let mut state = self.stacks.clone();

        for i in &self.instructions {
            let mut temp = Vec::new();
            for _ in 0..i.count {
                if let Some(c) = state[i.from - 1].pop() {
                    temp.push(c);
                }
            }
            temp.reverse();

            state[i.to - 1].extend(temp.iter());
        }

        state.iter().filter_map(|v| v.last()).collect()
    }

    fn number() -> u8 {
        5
    }
}

fn main() {
    Day05::run();
}
