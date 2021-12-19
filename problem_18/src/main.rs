mod ast;

use ast::SnailNumber;
use itertools::Itertools;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(snailnumbers);

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [problem_input]", args[0]);
        std::process::exit(-1);
    }

    let data = std::fs::read_to_string(&args[1]).expect("Unable to open input file");
    let parser = snailnumbers::SnailNumberParser::new();
    let numbers: Vec<Box<SnailNumber>> = data
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| parser.parse(s).expect("Unable to parse Snail number"))
        .collect();

    let answer_one = numbers
        .iter()
        .cloned()
        .reduce(|a, b| a + b)
        .unwrap()
        .magnitude();

    let answer_two = numbers.iter().permutations(2).fold(0_usize, |acc, v| {
        let a = v[0].clone();
        let b = v[1].clone();
        std::cmp::max(acc, (a + b).magnitude())
    });

    println!("{}", answer_one);
    println!("{}", answer_two);
}
