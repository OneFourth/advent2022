use util::*;

#[derive(Debug)]
struct Grid {
    map: Vec<Vec<u32>>,
    width: isize,
    height: isize,
}

type Position = (isize, isize);

impl Grid {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<_>> = input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let width = map[0].len() as isize;
        let height = map.len() as isize;

        Self { map, width, height }
    }

    fn get(&self, (x, y): Position) -> Option<u32> {
        self.map
            .get(y as usize)
            .and_then(|v| v.get(x as usize).copied())
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
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                let pos = (x, y);
                let value = self.grid.get(pos).unwrap();
                let valid = directions.iter().any(|direction| {
                    let mut it = GridIterator {
                        position: pos,
                        direction: *direction,
                        grid: &self.grid,
                    };

                    it.all(|c| c < value)
                });

                if valid {
                    valid_trees.push(pos);
                }
            }
        }

        valid_trees.len().to_string()
    }

    fn part2(&self) -> String {
        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        let mut scores = Vec::new();
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                let pos = (x, y);
                let value = self.grid.get(pos).unwrap();
                let score: usize = directions
                    .iter()
                    .map(|direction| {
                        let it = GridIterator {
                            position: pos,
                            direction: *direction,
                            grid: &self.grid,
                        };

                        let mut score = 0;
                        for v in it {
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
