use std::collections::HashSet;

use util::*;

type Point = (usize, usize);

struct Day14 {
    walls: Vec<(Point, Point)>,
}

impl Day14 {
    fn get_map(&self) -> HashSet<Point> {
        let mut map = HashSet::new();
        for &((mut x0, mut y0), (mut x1, mut y1)) in &self.walls {
            if y0 > y1 {
                std::mem::swap(&mut y0, &mut y1);
            }
            if x0 > x1 {
                std::mem::swap(&mut x0, &mut x1);
            }
            for y in y0..=y1 {
                for x in x0..=x1 {
                    map.insert((x, y));
                }
            }
        }

        map
    }
}

fn parse_point(input: &str) -> Point {
    input
        .split_once(',')
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .unwrap()
}

impl Day for Day14 {
    fn parse_input(input: &str) -> Self {
        let walls = input
            .lines()
            .flat_map(|l| {
                l.split(" -> ")
                    .collect::<Vec<_>>()
                    .windows(2)
                    .map(|s| (parse_point(s[0]), parse_point(s[1])))
                    .collect::<Vec<_>>()
            })
            .collect();

        Self { walls }
    }

    fn part1(&self) -> String {
        let map = self.get_map();
        let bottom = *map.iter().map(|(_, y)| y).max().unwrap();
        let mut sands = HashSet::new();

        'outer: loop {
            let mut sand = (500usize, 0);
            'falling: loop {
                let mut settled = true;
                for new_x in [sand.0, sand.0 - 1, sand.0 + 1] {
                    let new_sand = (new_x, sand.1 + 1);

                    if !map.contains(&new_sand) && !sands.contains(&new_sand) {
                        sand = new_sand;
                        settled = false;
                        break;
                    }
                }

                if settled {
                    sands.insert(sand);
                    break 'falling;
                } else if sand.1 >= bottom {
                    break 'outer;
                }
            }
        }

        sands.len().to_string()
    }

    fn part2(&self) -> String {
        let map = self.get_map();
        let bottom = *map.iter().map(|(_, y)| y).max().unwrap();
        let mut sands = HashSet::new();

        'outer: loop {
            let mut sand = (500usize, 0);
            'falling: loop {
                let mut settled = true;
                for new_x in [sand.0, sand.0 - 1, sand.0 + 1] {
                    let new_sand = (new_x, sand.1 + 1);

                    if !map.contains(&new_sand)
                        && !sands.contains(&new_sand)
                        && new_sand.1 < bottom + 2
                    {
                        sand = new_sand;
                        settled = false;
                        break;
                    }
                }

                if settled {
                    sands.insert(sand);
                    break 'falling;
                }
            }
            if sand == (500, 0) {
                break 'outer;
            }
        }

        sands.len().to_string()
    }

    fn number() -> u8 {
        14
    }
}

fn main() {
    Day14::run();
}
