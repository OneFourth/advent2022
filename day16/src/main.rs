use std::collections::{BTreeSet, HashMap};

use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use pathfinding::directed::astar::astar;
use pathfinding::directed::bfs::bfs;
use pathfinding::directed::dijkstra::dijkstra_all;

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
    total: isize,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>2}: total: {:>4} at {}, open:",
            self.time, self.total, self.position
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
    fn successors(&self, g: &HashMap<ValveName, Valve>) -> Vec<(Self, isize)> {
        let mut new_states = Vec::new();

        let pressure = g
            .iter()
            .filter_map(|(n, v)| (!self.open_valves.contains(n)).then_some(v.rate))
            .sum::<isize>();

        let v = g.get(&self.position).unwrap();

        let is_starting_place = self.position == ValveName::new("AA");
        for (name, cost) in &v.paths {
            let time = self.time + cost;
            if time <= 30 && *name != ValveName::new("AA") && !self.open_valves.contains(name) {
                let mut open_valves = self.open_valves.clone();
                open_valves.insert(*name);

                let next = g.get(name).unwrap();
                let flow = self.flow + next.rate;

                new_states.push((
                    Self {
                        position: *name,
                        open_valves,
                        flow,
                        time,
                        total: self.total + (cost * self.flow),
                    },
                    pressure * cost,
                ));
            }
        }

        if new_states.is_empty() && !is_starting_place && (self.time + 1) <= 30 {
            new_states.push((
                Self {
                    time: self.time + 1,
                    open_valves: self.open_valves.clone(),
                    total: self.total + self.flow,
                    ..*self
                },
                pressure,
            ));
        }

        /*
        println!("{self}");
        for (s, cost) in &new_states {
            println!("{cost:>4} {s}");
        }
        println!();
        */

        new_states
    }
}

#[derive(Debug)]
struct Valve {
    paths: HashMap<ValveName, isize>,
    rate: isize,
}

struct Day16 {
    valves: HashMap<ValveName, Valve>,
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

fn parse_valve(input: &str) -> IResult<&str, (ValveName, (Vec<ValveName>, isize))> {
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
        |(_, name, _, rate, _, paths)| (name, (paths, rate)),
    )(input)
}

impl Day for Day16 {
    fn parse_input(input: &str) -> Self {
        let whole_graph: HashMap<_, _> = input.lines().map(|l| parse_valve(l).unwrap().1).collect();
        let targets: Vec<_> = whole_graph
            .iter()
            .filter_map(|(name, &(_, rate))| {
                (rate > 0 || *name == ValveName::new("AA")).then_some(*name)
            })
            .collect();

        let mut valves = HashMap::new();
        for name in &targets {
            let paths = targets
                .iter()
                .filter(|n| *n != name)
                .map(|n| {
                    (
                        *n,
                        bfs(&name, |v| &whole_graph.get(v).unwrap().0, |v| *v == n)
                            .unwrap()
                            .len() as isize,
                    )
                })
                .collect();
            valves.insert(
                *name,
                Valve {
                    paths,
                    rate: whole_graph.get(name).unwrap().1,
                },
            );
        }

        Self { valves }
    }

    fn part1(&self) -> String {
        let start = State {
            position: ValveName::new("AA"),
            open_valves: BTreeSet::new(),
            flow: 0,
            time: 1,
            total: 0,
        };

        let path = astar(
            &start,
            |n| n.successors(&self.valves),
            |n| (n.time - 30) * n.flow,
            |n| n.time == 30,
        )
        .unwrap();

        for s in &path.0 {
            println!("{s}");
        }

        let last = path.0.last().unwrap();
        let released = last.total + last.flow;

        released.to_string()
    }

    fn part2(&self) -> String {
        let start = State {
            position: ValveName::new("AA"),
            open_valves: BTreeSet::new(),
            flow: 0,
            time: 5,
            total: 0,
        };

        let paths = dijkstra_all(&start, |n| n.successors(&self.valves));

        let mut best_paths = HashMap::new();
        for (s, _) in paths {
            let value = s.total + s.flow;
            let e = best_paths.entry(s.open_valves).or_insert(value);
            *e = value.max(*e);
        }

        let mut best_paths: Vec<_> = best_paths.iter().collect();
        best_paths.sort_unstable_by_key(|(_, c)| *c);
        best_paths.reverse();

        let mut max = 0;
        for (s, c) in &best_paths {
            for (_, c2) in best_paths.iter().filter(|(s2, _)| s.is_disjoint(s2)) {
                let total = *c + *c2;
                if total > max {
                    max = total;
                }
            }
        }

        max.to_string()
    }

    fn number() -> u8 {
        16
    }
}

fn main() {
    Day16::run();
}
