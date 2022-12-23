use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;
use util::*;

#[derive(Clone, PartialEq, Eq)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;

        let inner_comp = |l: &[Packet], r: &[Packet]| {
            let mut l = l.iter();
            let mut r = r.iter();

            loop {
                break match (l.next(), r.next()) {
                    (None, None) => Equal,
                    (None, Some(_)) => Less,
                    (Some(_), None) => Greater,
                    (Some(l), Some(r)) => {
                        let c = l.cmp(r);
                        if c == Equal {
                            continue;
                        }
                        c
                    }
                };
            }
        };

        match (self, other) {
            (Packet::Integer(l), Packet::Integer(r)) => {
                if l == r {
                    Equal
                } else {
                    l.cmp(r)
                }
            }
            (Packet::List(l), Packet::List(r)) => inner_comp(l, r),
            (l, Packet::List(r)) => inner_comp(&[l.clone()], r),
            (Packet::List(l), r) => inner_comp(l, &[r.clone()]),
        }
    }
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Integer(v) => write!(f, "{v}"),
            Packet::List(p) => {
                write!(f, "[")?;
                if !p.is_empty() {
                    write!(f, "{}", p[0])?;
                    for v in p.iter().skip(1) {
                        write!(f, ",{v}")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

fn parse_number(input: &str) -> IResult<&str, Packet> {
    map_res(digit1, |v: &str| v.parse::<u32>().map(Packet::Integer))(input)
}

fn parse_list(input: &str) -> IResult<&str, Packet> {
    map(
        delimited(
            tag("["),
            separated_list0(tag(","), alt((parse_number, parse_list))),
            tag("]"),
        ),
        Packet::List,
    )(input)
}

struct Day13 {
    packets: Vec<(Packet, Packet)>,
}

impl Day for Day13 {
    fn parse_input(input: &str) -> Self {
        let packets = input
            .split("\n\n")
            .map(|s| {
                let (first, second) = s.split_once('\n').unwrap();
                (parse_list(first).unwrap().1, parse_list(second).unwrap().1)
            })
            .collect();

        Self { packets }
    }

    fn part1(&self) -> String {
        self.packets
            .iter()
            .enumerate()
            .filter_map(|(i, (l, r))| if l < r { Some(i + 1) } else { None })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        let mut packets = Vec::new();
        let divider1 = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
        let divider2 = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);
        packets.push(divider1.clone());
        packets.push(divider2.clone());

        for (l, r) in &self.packets {
            packets.push(l.clone());
            packets.push(r.clone());
        }

        packets.sort();

        let key = (packets.iter().position(|p| *p == divider1).unwrap() + 1)
            * (packets.iter().position(|p| *p == divider2).unwrap() + 1);

        key.to_string()
    }

    fn number() -> u8 {
        13
    }
}

fn main() {
    Day13::run();
}
