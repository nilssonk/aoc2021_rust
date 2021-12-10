use std::collections::HashMap;

fn score_line(input: &str) -> (usize, usize) {
    let corrupt_map = |c| match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    };
    let closer_map = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    let mut expected_closers = Vec::new();

    for c in input.chars() {
        if let Some(&closer) = closer_map.get(&c) {
            expected_closers.push(closer);
        } else if let Some(expected) = expected_closers.pop() {
            if c != expected {
                return (corrupt_map(c), 0);
            }
        } else {
            return (corrupt_map(c), 0);
        }
    }

    // Line is only incomplete
    let incomplete_map = |c| match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    };

    let incomplete_score = expected_closers
        .into_iter()
        .rev()
        .fold(0, |acc, c| acc * 5 + incomplete_map(c));

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
