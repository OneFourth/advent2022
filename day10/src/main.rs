use util::*;

enum Instruction {
    Add(isize),
    Noop,
}

impl Instruction {
    fn new(input: &str) -> Self {
        let mut s = input.split(' ');
        let ins = s.next().unwrap();
        let v = s.next();
        match (ins, v) {
            ("noop", None) => Instruction::Noop,
            ("addx", Some(v)) => Instruction::Add(v.parse().unwrap()),
            _ => panic!("Invalid"),
        }
    }
}

struct Day10 {
    instructions: Vec<Instruction>,
}

struct Computer {
    cycle: isize,
    x: isize,
    signals: Vec<isize>,
    image: [char; 40 * 6],
}

impl Computer {
    fn new() -> Self {
        Self {
            cycle: 0,
            x: 1,
            signals: Vec::new(),
            image: [' '; 40 * 6],
        }
    }

    fn check_cycle(&mut self) {
        self.cycle += 1;
        if self.cycle == 20 || self.cycle % 40 == 20 {
            self.signals.push(self.cycle * self.x);
        }

        let cycle = (self.cycle - 1) % 40;
        let xs = [self.x - 1, self.x, self.x + 1];

        if let Some(p) = self.image.get_mut((self.cycle - 1) as usize) {
            if xs.iter().any(|x| *x == cycle) {
                *p = '#'
            } else {
                *p = '.'
            }
        };
    }

    fn run(&mut self, instructions: &[Instruction]) {
        for i in instructions {
            match i {
                Instruction::Add(v) => {
                    self.check_cycle();
                    self.check_cycle();
                    self.x += v
                }
                Instruction::Noop => {
                    self.check_cycle();
                }
            }
        }
    }
}

impl std::fmt::Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in self.image.chunks_exact(40) {
            for c in row {
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl Day for Day10 {
    fn parse_input(input: &str) -> Self {
        let instructions = input.lines().map(Instruction::new).collect();

        Self { instructions }
    }

    fn part1(&self) -> String {
        let mut computer = Computer::new();
        computer.run(&self.instructions);

        computer.signals.iter().take(6).sum::<isize>().to_string()
    }

    fn part2(&self) -> String {
        let mut computer = Computer::new();
        computer.run(&self.instructions);

        computer.to_string()
    }

    fn number() -> u8 {
        10
    }
}

fn main() {
    Day10::run();
}
