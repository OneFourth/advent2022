use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res, opt, recognize};
use nom::sequence::tuple;
use nom::IResult;

use util::*;

type Point = (isize, isize);

#[derive(Debug)]
struct Sensor {
    position: Point,
    closest_beacon: Point,
}

fn distance((x0, y0): Point, (x1, y1): Point) -> usize {
    x0.abs_diff(x1) + y0.abs_diff(y1)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    LU,
    UR,
    RD,
    DL,
}

struct RingIter {
    center: Point,
    radius: isize,
    direction: Direction,
    curr: Option<Point>,
}

impl Iterator for RingIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let start = (self.center.0 - self.radius, self.center.1);
        match self.direction {
            Direction::LU => {
                if self.curr.is_none() {
                    self.curr = Some(start);
                } else {
                    self.curr = self.curr.map(|(x, y)| (x + 1, y - 1));
                }

                if self.curr.unwrap().0 == self.center.0 {
                    self.direction = Direction::UR;
                }
            }
            Direction::UR => {
                self.curr = self.curr.map(|(x, y)| (x + 1, y + 1));
                if self.curr.unwrap().1 == self.center.1 {
                    self.direction = Direction::RD;
                }
            }
            Direction::RD => {
                self.curr = self.curr.map(|(x, y)| (x - 1, y + 1));
                if self.curr.unwrap().0 == self.center.0 {
                    self.direction = Direction::DL;
                }
            }
            Direction::DL => {
                if self.curr.is_some() {
                    self.curr = self.curr.map(|(x, y)| (x - 1, y - 1));
                    if self.curr.unwrap() == start {
                        self.curr = None;
                    }
                }
            }
        }

        self.curr
    }
}

impl Sensor {
    fn get_range(&self) -> usize {
        distance(self.position, self.closest_beacon)
    }

    fn contains(&self, p: Point) -> bool {
        p != self.closest_beacon && distance(self.position, p) <= self.get_range()
    }

    fn get_extents(&self) -> [Point; 4] {
        let d = self.get_range() as isize;
        [
            (self.position.0, self.position.1 - d),
            (self.position.0 + d, self.position.1),
            (self.position.0, self.position.1 + d),
            (self.position.0 - d, self.position.1),
        ]
    }

    fn iter_ring(&self) -> RingIter {
        RingIter {
            center: self.position,
            radius: (self.get_range() + 1) as isize,
            direction: Direction::LU,
            curr: None,
        }
    }
}

fn parse_number(input: &str) -> IResult<&str, isize> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), |number: &str| {
        number.parse::<isize>()
    })(input)
}

fn parse_coord(input: &str) -> IResult<&str, Point> {
    map(
        tuple((tag("x="), parse_number, tag(", y="), parse_number)),
        |(_, x, _, y)| (x, y),
    )(input)
}

fn parse_sensor(input: &str) -> IResult<&str, Sensor> {
    map(
        tuple((
            tag("Sensor at "),
            parse_coord,
            tag(": closest beacon is at "),
            parse_coord,
        )),
        |(_, position, _, closest_beacon)| Sensor {
            position,
            closest_beacon,
        },
    )(input)
}

struct Day15 {
    sensors: Vec<Sensor>,
}

impl Day15 {
    fn get_area(&self) -> (Point, Point) {
        let e = self.sensors[0].get_extents();
        let mut min = (
            *e.iter().map(|(x, _)| x).min().unwrap(),
            *e.iter().map(|(_, y)| y).min().unwrap(),
        );
        let mut max = (
            *e.iter().map(|(x, _)| x).max().unwrap(),
            *e.iter().map(|(_, y)| y).max().unwrap(),
        );

        for s in &self.sensors {
            let e = s.get_extents();
            min.0 = min.0.min(*e.iter().map(|(x, _)| x).min().unwrap());
            min.1 = min.1.min(*e.iter().map(|(_, y)| y).min().unwrap());
            max.0 = max.0.max(*e.iter().map(|(x, _)| x).max().unwrap());
            max.1 = max.1.max(*e.iter().map(|(_, y)| y).max().unwrap());
        }

        (min, max)
    }
}

impl Day for Day15 {
    fn parse_input(input: &str) -> Self {
        let sensors = input.lines().map(|l| parse_sensor(l).unwrap().1).collect();

        Self { sensors }
    }

    fn part1(&self) -> String {
        let ((min_x, _), (max_x, _)) = self.get_area();

        let mut count = 0;
        for x in min_x..=max_x {
            let p = (x, 2_000_000);
            if self.sensors.iter().any(|s| s.contains(p)) {
                count += 1;
            }
        }

        count.to_string()
    }

    fn part2(&self) -> String {
        let mut tuning_freq = 0;
        for s in &self.sensors {
            for p in s.iter_ring() {
                if p.0 >= 0
                    && p.0 <= 4_000_000
                    && p.1 >= 0
                    && p.1 <= 4_000_000
                    && self
                        .sensors
                        .iter()
                        .all(|s| !s.contains(p) && s.closest_beacon != p)
                {
                    tuning_freq = p.0 * 4_000_000 + p.1;
                    break;
                }
            }
        }

        tuning_freq.to_string()
    }

    fn number() -> u8 {
        15
    }
}

fn main() {
    Day15::run();
}
