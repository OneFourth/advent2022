use std::path::PathBuf;
use std::time::Instant;

use owo_colors::OwoColorize;

pub trait Day: Sized {
    fn parse_input(input: &str) -> Self;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
    fn number() -> u8;

    fn run() {
        println!("Day {}", Self::number().bright_red());

        let now = Instant::now();
        let p = PathBuf::from(format!("input/{:02}", Self::number()));

        println!("Reading {}", p.display().bright_red());

        let input = std::fs::read_to_string(p).unwrap();

        let d = Self::parse_input(&input);
        println!(
            "Input parsing took {} ms",
            now.elapsed().as_millis().bright_yellow()
        );

        let now = Instant::now();
        let part1 = d.part1();

        println!(
            "Part 1: {}, took {} ms",
            part1.bright_blue(),
            now.elapsed().as_millis().bright_yellow()
        );

        let now = Instant::now();
        let part2 = d.part2();

        println!(
            "Part 2: {}, took {} ms",
            part2.bright_blue(),
            now.elapsed().as_millis().bright_yellow()
        );
    }
}
