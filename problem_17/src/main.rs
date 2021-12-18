#![feature(map_first_last)]

use regex::Regex;
use std::collections::BTreeSet;

type Integer = i16;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [problem_input]", args[0]);
        std::process::exit(-1);
    }

    let data = std::fs::read_to_string(&args[1]).expect("Unable to open input file");
    let mut rows = data.split('\n').filter(|s| !s.is_empty());

    let input = rows.next().expect("Input is empty");

    let re = Regex::new(
        r".+x=(?P<xmin>[0-9-]+)\.\.(?P<xmax>[0-9-]+).+y=(?P<ymin>[0-9-]+)\.\.(?P<ymax>[0-9-]+)",
    )
    .unwrap();

    let caps = re.captures(input).unwrap();
    let parse_coord = |coord| {
        caps.name(coord)
            .unwrap()
            .as_str()
            .parse::<Integer>()
            .expect("Unable to parse integer")
    };
    let xmin = parse_coord("xmin");
    let xmax = parse_coord("xmax");
    let ymin = parse_coord("ymin");
    let ymax = parse_coord("ymax");

    // Inversion of the arithmetic sum over y
    let n_limit = |p0, v0| {
        let b = 2.0 * v0 as f32 + 1.0;
        let left = 0.5 * b;
        let right: f32 = 0.5 * (b * b - 8.0 * p0 as f32).sqrt();
        (left + right).ceil() as Integer
    };

    // Arithmetic sums over y and x, respectively
    let pos_after_n = |v0y, n| (n * (2 * v0y + 1) - n * n) / 2;
    let pos_after_n_clamped = |v0x, n| {
        if n >= v0x {
            v0x * (v0x + 1) / 2
        } else {
            pos_after_n(v0x, n)
        }
    };

    // Inversion of the arithmetic sum over x
    let min_vx = (-0.5 + (0.25 + xmin as f32).sqrt()).floor() as Integer;
    let max_vx = xmax;
    let min_vy = ymin;
    let max_vy = ymin.abs();

    let mut valid_vs = BTreeSet::new();
    for vy in min_vy..(max_vy + 1) {
        for vx in min_vx..(max_vx + 1) {
            for n in 0..(n_limit(ymin, vy) + 1) {
                let x = pos_after_n_clamped(vx, n);
                if x > xmax {
                    break;
                }
                let y = pos_after_n(vy, n);

                if x >= xmin && x <= xmax && y >= ymin && y <= ymax {
                    valid_vs.insert((vy, vx));
                }
            }
        }
    }

    let answer_one = valid_vs
        .last()
        .and_then(|&(vy, _)| Some(pos_after_n(vy, vy)))
        .unwrap();
    let answer_two = valid_vs.len();

    println!("{}", answer_two);
    println!("{}", answer_one);
}
