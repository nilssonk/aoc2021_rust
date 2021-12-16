use fnv::{FnvHashMap, FnvHashSet};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};
use std::iter::FromIterator;

use common::Array2D;

type Integer = i16;
type Vec2 = (Integer, Integer);

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
        rows.flat_map(|s| s.chars()).map(|c| c as u8 - '0' as u8),
        width_one,
    );
}
