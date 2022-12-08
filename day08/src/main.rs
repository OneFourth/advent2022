use util::*;

#[derive(Debug)]
struct Grid {
    map: Vec<Vec<u32>>,
}

type Position = (isize, isize);

impl Grid {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<_>> = input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        Self { map }
    }

    fn get(&self, (x, y): Position) -> Option<u32> {
        self.map
            .get(y as usize)
            .and_then(|v| v.get(x as usize).copied())
    }

    fn positions(&self) -> GridPosIterator {
        GridPosIterator {
            position: (0, 0),
            grid: self,
        }
    }

    fn direction_iter(&self, position: Position, direction: Position) -> GridIterator {
        GridIterator {
            position,
            direction,
            grid: self,
        }
    }
}

#[derive(Debug)]
struct GridPosIterator<'a> {
    position: Position,
    grid: &'a Grid,
}

impl<'a> Iterator for GridPosIterator<'a> {
    type Item = (Position, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let x_move = (self.position.0 + 1, self.position.1);
        let y_move = (0, self.position.1 + 1);
        if let Some(v) = self.grid.get(x_move) {
            self.position = x_move;
            Some((x_move, v))
        } else if let Some(v) = self.grid.get(y_move) {
            self.position = y_move;
            Some((y_move, v))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct GridIterator<'a> {
    position: Position,
    direction: Position,
    grid: &'a Grid,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.position.0 += self.direction.0;
        self.position.1 += self.direction.1;

        self.grid.get(self.position)
    }
}

struct Day08 {
    grid: Grid,
}

impl Day for Day08 {
    fn parse_input(input: &str) -> Self {
        Self {
            grid: Grid::new(input),
        }
    }

    fn part1(&self) -> String {
        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        let mut valid_trees = Vec::new();

        for (pos, value) in self.grid.positions() {
            let valid = directions
                .iter()
                .any(|&dir| self.grid.direction_iter(pos, dir).all(|c| c < value));

            if valid {
                valid_trees.push(pos);
            }
        }

        valid_trees.len().to_string()
    }

    fn part2(&self) -> String {
        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        let mut scores = Vec::new();
        for (pos, value) in self.grid.positions() {
            let score: usize = directions
                .iter()
                .map(|&dir| {
                    let mut score = 0;
                    for v in self.grid.direction_iter(pos, dir) {
                        if v < value {
                            score += 1;
                        } else {
                            score += 1;
                            break;
                        }
                    }
                    score
                })
                .product();

            scores.push(score);
        }

        scores.iter().max().unwrap().to_string()
    }

    fn number() -> u8 {
        8
    }
}

fn main() {
    Day08::run();
}
