use std::collections::HashMap;

use pathfinding::directed::astar::astar;

use util::*;

struct Day12 {
    map: Vec<Vec<u8>>,
    starts: Vec<Point>,
    end: Point,
}

type Point = (usize, usize);
type Graph = HashMap<Point, Vec<Point>>;

impl Day12 {
    fn get(&self, (x, y): Point) -> Option<&u8> {
        match self.map.get(y).and_then(|v| v.get(x)) {
            Some(b'S') => Some(&b'a'),
            Some(b'E') => Some(&b'z'),
            c => c,
        }
    }

    fn get_sides((x, y): Point) -> [Point; 4] {
        [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
    }

    fn get_graph(&self) -> Graph {
        let mut g = HashMap::new();
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                let curr = (x, y);
                let el = self.get(curr).unwrap();
                for side in Self::get_sides(curr) {
                    if let Some(next_el) = self.get(side) {
                        if next_el <= el || next_el - el == 1 {
                            g.entry(curr).or_insert_with(Vec::new).push(side);
                        }
                    }
                }
            }
        }

        g
    }
}

impl Day for Day12 {
    fn parse_input(input: &str) -> Self {
        let map: Vec<Vec<_>> = input.lines().map(|s| s.bytes().collect()).collect();
        let mut starts = Vec::new();
        let mut end = (0, 0);
        for (y, v) in map.iter().enumerate() {
            for (x, c) in v.iter().enumerate() {
                match c {
                    b'S' => starts.insert(0, (x, y)),
                    b'E' => end = (x, y),
                    b'a' => starts.push((x, y)),
                    _ => {}
                }
            }
        }

        Self { map, starts, end }
    }

    fn part1(&self) -> String {
        let g = self.get_graph();

        let path = astar(
            &self.starts[0],
            |p| {
                g.get(p)
                    .map(|v| v.iter().map(|v| (*v, 1)).collect::<Vec<_>>())
                    .unwrap_or_default()
            },
            |p| self.end.0.abs_diff(p.0) + self.end.1.abs_diff(p.1),
            |p| *p == self.end,
        )
        .unwrap();

        path.1.to_string()
    }

    fn part2(&self) -> String {
        let g = self.get_graph();

        self.starts
            .iter()
            .filter_map(|&start| {
                astar(
                    &start,
                    |p| {
                        g.get(p)
                            .map(|v| v.iter().map(|v| (*v, 1)).collect::<Vec<_>>())
                            .unwrap_or_default()
                    },
                    |p| self.end.0.abs_diff(p.0) + self.end.1.abs_diff(p.1),
                    |p| *p == self.end,
                )
                .map(|path| path.1)
            })
            .min()
            .unwrap()
            .to_string()
    }

    fn number() -> u8 {
        12
    }
}

fn main() {
    Day12::run();
}
