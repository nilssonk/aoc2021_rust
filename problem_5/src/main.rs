#![feature(step_trait)]

mod geometry;
mod plot;

use crate::geometry::*;
use crate::plot::*;

// Traits
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::Send;
use std::marker::Sync;
use std::str::FromStr;

// Other
use rayon::prelude::*;
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

fn solve<'a, I, T, F>(input: I, predicate: F) -> usize
where
    I: ParallelIterator<Item = &'a Line<T>>,
    T: PlottingNumber + Send,
    F: Fn(&&'a Line<T>) -> bool + Sync,
    Vec2<T>: Hash,
{
    input
        .filter(&predicate)
        .fold(
            || HashMap::<Vec2<T>, usize>::new(),
            |mut m, l| {
                for p in plot_line(l) {
                    let e = m.entry(p).or_default();
                    *e += 1;
                }

                m
            },
        )
        .reduce_with(|mut m1, m2| {
            for (k, v) in m2 {
                let e = m1.entry(k).or_default();
                *e += v;
            }
            m1
        })
        .unwrap()
        .into_par_iter()
        .filter(|&(_, v)| v >= 2)
        .count()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [problem_input]", args[0]);
        std::process::exit(-1);
    }

    let data = std::fs::read_to_string(&args[1]).expect("Unable to open input file");

    let lines: Vec<Line<i32>> = parse_lines::<i32>(&data).collect();

    let answer_one = solve(lines.par_iter(), |(a, b)| a.x == b.x || a.y == b.y);
    let answer_two = solve(lines.par_iter(), |_| true);

    println!("{}", answer_one);
    println!("{}", answer_two);
}
