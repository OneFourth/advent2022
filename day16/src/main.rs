use std::collections::{BTreeSet, HashMap};

use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use pathfinding::directed::astar::astar;

use util::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct ValveName {
    name: [char; 2],
}

impl ValveName {
    fn new(s: &str) -> Self {
        let mut n = s.chars();
        Self {
            name: [n.next().unwrap(), n.next().unwrap()],
        }
    }
}

impl std::fmt::Display for ValveName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.name[0], self.name[1])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    position: ValveName,
    open_valves: BTreeSet<ValveName>,
    flow: isize,
    time: isize,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>2}: {:>4} at {}, open:",
            self.time, self.flow, self.position
        )?;

        for (i, n) in self.open_valves.iter().enumerate() {
            if i == 0 {
                write!(f, " {}", n)?;
            } else {
                write!(f, ", {}", n)?;
            }
        }

        Ok(())
    }
}

impl State {
    fn successors(&self, g: &Graph) -> Vec<(Self, isize)> {
        let mut new_states = Vec::new();

        let v = g.get(&self.position).unwrap();

        let pressure = g
            .iter()
            .filter_map(|(n, v)| (!self.open_valves.contains(n)).then_some(v.rate))
            .sum::<isize>();

        if v.rate > 0 && !self.open_valves.contains(&self.position) {
            let mut open_valves = self.open_valves.clone();
            open_valves.insert(self.position);

            new_states.push((
                Self {
                    position: self.position,
                    open_valves,
                    flow: self.flow + v.rate,
                    time: self.time + 1,
                },
                pressure,
            ));
        }

        for p in &v.paths {
            new_states.push((
                Self {
                    position: *p,
                    open_valves: self.open_valves.clone(),
                    flow: self.flow,
                    time: self.time + 1,
                },
                pressure,
            ));
        }

        new_states
    }
}

struct Valve {
    paths: Vec<ValveName>,
    rate: isize,
}

type Graph = HashMap<ValveName, Valve>;

struct Day16 {
    valves: Graph,
}

fn parse_number(input: &str) -> IResult<&str, isize> {
    map_res(digit1, |v: &str| v.parse::<isize>())(input)
}

fn parse_name(input: &str) -> IResult<&str, ValveName> {
    map(take(2usize), |n| ValveName::new(n))(input)
}

fn parse_list(input: &str) -> IResult<&str, Vec<ValveName>> {
    separated_list1(tag(", "), parse_name)(input)
}

fn parse_valve(input: &str) -> IResult<&str, (ValveName, Valve)> {
    map(
        tuple((
            tag("Valve "),
            parse_name,
            tag(" has flow rate="),
            parse_number,
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            parse_list,
        )),
        |(_, name, _, rate, _, paths)| (name, Valve { paths, rate }),
    )(input)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Turn {
    Mine,
    Elephant,
}

impl Day for Day16 {
    fn parse_input(input: &str) -> Self {
        let valves = input.lines().map(|l| parse_valve(l).unwrap().1).collect();

        Self { valves }
    }

    fn part1(&self) -> String {
        let start = State {
            position: ValveName::new("AA"),
            open_valves: BTreeSet::new(),
            flow: 0,
            time: 1,
        };

        let path = astar(
            &start,
            |n| n.successors(&self.valves),
            |n| (n.time - 30) * n.flow,
            |n| n.time == 30,
        )
        .unwrap();

        let mut released = 0;
        for p in &path.0 {
            println!("{p}");
            released += p.flow;
        }

        released.to_string()
    }

    fn part2(&self) -> String {
        let start = (
            State {
                position: ValveName::new("AA"),
                open_valves: BTreeSet::new(),
                flow: 0,
                time: 5,
            },
            ValveName::new("AA"),
            Turn::Mine,
        );

        let path = astar(
            &start,
            |(n, eleph_position, turn)| match turn {
                Turn::Mine => n
                    .successors(&self.valves)
                    .into_iter()
                    .map(|(mut s, cost)| {
                        s.time -= 1;
                        ((s.clone(), *eleph_position, Turn::Elephant), cost)
                    })
                    .collect::<Vec<_>>(),

                Turn::Elephant => {
                    static mut COUNT: isize = 0;
                    let mut e = n.clone();
                    let my_position = std::mem::replace(&mut e.position, *eleph_position);
                    e.successors(&self.valves)
                        .into_iter()
                        .map(|(mut s, cost)| {
                            unsafe {
                                if s.time > COUNT {
                                    COUNT = s.time;
                                    dbg!(COUNT);
                                }
                            };
                            let eleph_position = s.position;
                            s.position = my_position;
                            ((s.clone(), eleph_position, Turn::Mine), cost)
                        })
                        .collect()
                }
            },
            |(n, _, _)| (n.time - 26) * n.flow,
            |(n, _, t)| (n.time == 30 && *t == Turn::Elephant),
        )
        .unwrap();

        let mut released = 0;
        for v in path.0.chunks_exact(2) {
            if let [(s1, _, _), (s2, e, _)] = v {
                println!("\t{s1}");
                let mut s = s2.clone();
                s.position = *e;
                println!("\t{s}");
                released += s1.flow;
            }
        }

        released.to_string()
    }

    fn number() -> u8 {
        16
    }
}

fn main() {
    Day16::run();
}
