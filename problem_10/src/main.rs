#[macro_use]
extern crate lazy_static;

use fnv::FnvHashMap;

fn score_line(input: &str) -> (usize, usize) {
    lazy_static! {
        static ref CORRUPT_MAP: FnvHashMap<char, usize> =
            [(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
                .iter()
                .cloned()
                .collect();
        static ref INCOMPLETE_MAP: FnvHashMap<char, usize> =
            [(')', 1), (']', 2), ('}', 3), ('>', 4)]
                .iter()
                .cloned()
                .collect();
        static ref CLOSER_MAP: FnvHashMap<char, char> =
            [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
                .iter()
                .cloned()
                .collect();
    }

    let corrupt = |x| CORRUPT_MAP.get(&x).cloned().unwrap_or(0);
    let incomplete = |x| INCOMPLETE_MAP.get(&x).cloned().unwrap_or(0);

    let mut expected_closers = Vec::new();

    for c in input.chars() {
        if let Some(&closer) = CLOSER_MAP.get(&c) {
            expected_closers.push(closer);
        } else if let Some(expected) = expected_closers.pop() {
            if c != expected {
                return (corrupt(c), 0);
            }
        } else {
            return (corrupt(c), 0);
        }
    }

    // Line is only incomplete
    let incomplete_score = expected_closers
        .into_iter()
        .rev()
        .fold(0, |acc, c| acc * 5 + incomplete(c));

    (0, incomplete_score)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [problem_input]", args[0]);
        std::process::exit(-1);
    }

    let data = std::fs::read_to_string(&args[1]).expect("Unable to open input file");
    let rows = data.split('\n').filter(|s| !s.is_empty());

    let (corrupted, mut incomplete): (Vec<_>, Vec<_>) = rows.map(|s| score_line(s)).unzip();

    let answer_one = corrupted.iter().sum::<usize>();

    incomplete.retain(|&x| x != 0);
    incomplete.sort();
    let answer_two = incomplete[incomplete.len() / 2];

    println!("{}", answer_one);
    println!("{}", answer_two);
}
