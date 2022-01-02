extern crate nalgebra as na;
use na::{Rotation3, Vector3};

type Integer = i16;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [problem_input]", args[0]);
        std::process::exit(-1);
    }

    let data = std::fs::read_to_string(&args[1]).expect("Unable to open input file");
    let entries: Vec<(Integer, Vec<Vector3<Integer>>)> = data
        .split("\n\n")
        .map(|s| {
            let mut rows = s.split('\n').filter(|s| !s.is_empty());
            let scanner = rows
                .next()
                .unwrap()
                .split(' ')
                .nth(2)
                .unwrap()
                .parse::<Integer>()
                .unwrap();
            let readings: Vec<Vector3<Integer>> = rows
                .map(|s| {
                    Vector3::from_iterator(s.split(',').map(|s| s.parse::<Integer>().unwrap()))
                })
                .collect();
            (scanner, readings)
        })
        .collect();

    println!("{:?}", entries);
}
