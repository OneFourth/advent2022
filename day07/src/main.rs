use std::cell::Cell;
use std::collections::HashMap;

use util::*;

#[allow(non_camel_case_types)]
enum Terminal {
    cd_up,
    cd_home,
    cd { path: String },
    ls,
    dir { path: String },
    file { name: String, size: usize },
}

#[derive(Debug)]
enum Type {
    Folder(Cell<usize>),
    File(Cell<usize>),
}

impl Type {
    fn get_size(&self) -> &Cell<usize> {
        match self {
            Type::Folder(s) => s,
            Type::File(s) => s,
        }
    }
}

struct Day07 {
    lines: Vec<Terminal>,
}

fn get_filesystem(lines: &[Terminal]) -> HashMap<Vec<String>, Type> {
    let mut pwd = Vec::new();
    let mut fs = HashMap::new();
    fs.insert(pwd.clone(), Type::Folder(Cell::new(0)));

    for l in lines {
        match l {
            Terminal::cd_up => {
                pwd.pop();
            }
            Terminal::cd_home => {
                pwd = Vec::new();
            }
            Terminal::cd { path } => {
                pwd.push(path.clone());
            }
            Terminal::ls => {}
            Terminal::dir { path } => {
                let mut full_path = pwd.clone();
                full_path.push(path.clone());
                fs.insert(full_path.clone(), Type::Folder(Cell::new(0)));
            }
            Terminal::file { name, size } => {
                let mut path = pwd.clone();
                path.push(name.clone());
                fs.insert(path, Type::File(Cell::new(*size)));
            }
        }
    }

    for (path, t) in &fs {
        if let Type::File(size) = t {
            let mut parent = path.clone();
            while !parent.is_empty() {
                parent.pop();
                let (_, folder) = fs.iter().find(|&(p, _)| p == &parent).unwrap();
                folder.get_size().set(folder.get_size().get() + size.get());
            }
        }
    }

    fs
}

impl Day for Day07 {
    fn parse_input(input: &str) -> Self {
        let lines = input
            .lines()
            .map(|l| {
                let mut ws = l.split_whitespace();
                match (ws.next().unwrap(), ws.next().unwrap(), ws.next()) {
                    ("$", "ls", None) => Terminal::ls,
                    ("$", "cd", Some("/")) => Terminal::cd_home,
                    ("$", "cd", Some("..")) => Terminal::cd_up,
                    ("$", "cd", Some(p)) => Terminal::cd { path: p.to_owned() },
                    ("dir", p, None) => Terminal::dir { path: p.to_owned() },
                    (s, p, None) => Terminal::file {
                        name: p.to_owned(),
                        size: s.parse().unwrap(),
                    },
                    _ => panic!("Unsupported {l}"),
                }
            })
            .collect();

        Self { lines }
    }

    fn part1(&self) -> String {
        let fs = get_filesystem(&self.lines);

        fs.iter()
            .filter_map(|(_, t)| {
                if let Type::Folder(s) = t {
                    if s.get() <= 100_000 {
                        Some(s.get())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        let fs = get_filesystem(&self.lines);

        let total_used = fs.get(&Vec::new()).unwrap().get_size().get();
        let total_free = 70_000_000 - total_used;
        let needed = 30_000_000 - total_free;

        fs.iter()
            .filter_map(|(_, t)| {
                if let Type::Folder(s) = t {
                    if s.get() >= needed {
                        Some(s.get())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .min()
            .unwrap()
            .to_string()
    }

    fn number() -> u8 {
        7
    }
}

fn main() {
    Day07::run();
}
