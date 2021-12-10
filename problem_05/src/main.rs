#![feature(step_trait)]

mod geometry;
mod plot;

use crate::geometry::*;
use crate::plot::*;

// Traits
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

// Other
use std::collections::HashMap;

fn parse_lines<T>(input: &String) -> impl std::iter::Iterator<Item = Line<T>> + '_
where
    T: FromStr,
    T::Err: Debug,
{
    let re = regex::Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    input.lines().filter(|s| !s.is_empty()).map(move |s| {
        let caps = re.captures(s).expect("Regex capture failed");

        let parse = |i: usize| caps[i].parse::<T>().expect("Unable to parse integer at {}");

        (
            Vec2::<T> {
                x: parse(1),
                y: parse(2),
            },
            Vec2::<T> {
                x: parse(3),
                y: parse(4),
            },
        )
    })
}

fn solve<'a, T>(input: impl Iterator<Item = &'a Line<T>>) -> usize
where
    T: 'static + PlottingNumber,
    Vec2<T>: Hash,
{
    let mut counts: HashMap<Vec2<T>, u8> = HashMap::new();

    for line in input {
        for p in plot_line(&line) {
            let e = counts.entry(p).or_default();
            *e += 1;
        }
    }

    counts.values().filter(|&&v| v >= 2).count()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [problem_input]", args[0]);
        std::process::exit(-1);
    }

    let data = std::fs::read_to_string(&args[1]).expect("Unable to open input file");

    let lines: Vec<Line<i32>> = parse_lines::<i32>(&data).collect();

    let answer_one = solve(lines.iter().filter(|(a, b)| a.x == b.x || a.y == b.y));
    let answer_two = solve(lines.iter());

    println!("{}", answer_one);
    println!("{}", answer_two);
}
