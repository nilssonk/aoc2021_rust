#![feature(int_abs_diff)]

use fnv::FnvHashMap;
use fnv::FnvHashSet;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::FromIterator;

use common::Array2D;

type Integer = i16;
type Vec2 = (Integer, Integer);

fn manhattan_distance(a: &Vec2, b: &Vec2) -> Integer {
    let dx = a.0.abs_diff(b.0);
    let dy = a.1.abs_diff(b.1);

    (dx + dy) as Integer
}

fn a_star(input_at: impl Fn(Vec2) -> Option<u8>, start: Vec2, end: Vec2) -> Option<Integer> {
    let start_risk = manhattan_distance(&start, &end);

    let mut risks = FnvHashMap::from_iter([(start, Integer::from(0_u8))]);
    let mut candidates = FnvHashSet::from_iter([start]);
    let mut candidate_queue = BinaryHeap::from([Reverse((start_risk, start))]);

    while let Some(Reverse((risk, v @ (x, y)))) = candidate_queue.pop() {
        if v == end {
            return Some(risk);
        }

        let neighbors = [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)];
        for n in neighbors {
            if let Some(edge_risk) = input_at(n) {
                let estimated_risk = risks[&v] + edge_risk as Integer;
                let e = risks.entry(n).or_insert(Integer::MAX);
                if estimated_risk < *e {
                    *e = estimated_risk;
                    let heuristic = estimated_risk + manhattan_distance(&n, &end);
                    if candidates.insert(n) {
                        candidate_queue.push(Reverse((heuristic, n)));
                    }
                }
            }
        }

        candidates.remove(&v);
    }

    None
}

fn solve_one(input: &Array2D<u8, Integer>) -> Integer {
    let get_input_one = |v| input.get(v).copied();
    a_star(get_input_one, (0, 0), (input.width - 1, input.height - 1)).expect("No path exists")
}

fn solve_two(input: &Array2D<u8, Integer>) -> Integer {
    // The input data is repeated 4 times in each direction
    let new_width = input.width * 5;
    let new_height = input.height * 5;

    // The value increases by the x and y tile number but rolls over from 9 to 1
    let transform = |mut n: u8, x_tile, y_tile| {
        n += x_tile as u8 + y_tile as u8;
        n = n.saturating_sub(1);
        n %= 9;
        n += 1;
        n
    };

    // Simulate tile expansion to save memory
    let get_input_two = |(v_x, v_y)| {
        let v = (v_x % input.width, v_y % input.height);
        let x_tile = v_x / input.width;
        let y_tile = v_y / input.height;

        input
            .get(v)
            .copied()
            .map(|input| transform(input, x_tile, y_tile))
    };

    a_star(get_input_two, (0, 0), (new_width - 1, new_height - 1)).expect("No path exists")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [problem_input]", args[0]);
        std::process::exit(-1);
    }

    let data = std::fs::read_to_string(&args[1]).expect("Unable to open input file");
    let mut rows = data.split('\n').filter(|s| !s.is_empty()).peekable();

    // Peek first element to ascertain width
    let first = rows.peek().expect("Input is empty");
    let width_one = first.len() as Integer;

    let map = Array2D::from_iter(
        rows.flat_map(|s| s.chars()).map(|c| c as u8 - b'0'),
        width_one,
    );

    let answer_one = solve_one(&map);
    let answer_two = solve_two(&map);

    println!("{}", answer_one);
    println!("{}", answer_two);
}
