#[macro_use]
extern crate lazy_static;

use fnv::FnvHashMap;
use fnv::FnvHashSet;

use itertools::Itertools;
use std::collections::VecDeque;
use std::iter::FromIterator;

lazy_static! {
    static ref CHAR_TO_DIGIT: FnvHashMap<char, i8> = [
        ('a', 0),
        ('b', 1),
        ('c', 2),
        ('d', 3),
        ('e', 4),
        ('f', 5),
        ('g', 6),
    ].iter().cloned().collect();
    static ref DIGIT_TO_CHAR: FnvHashMap<i8, char> = {
        // Inverse of CHAR_TO_NUM
        let mut result = FnvHashMap::default();
        for (&k, &v) in CHAR_TO_DIGIT.iter() {
            result.insert(v, k);
        }
        result
    };
    static ref STR_TO_SEGMENT_VALUE: FnvHashMap<String, u8> = [
        ("abcefg".to_owned(), 0),
        ("cf".to_owned(), 1),
        ("acdeg".to_owned(), 2),
        ("acdfg".to_owned(), 3),
        ("bcdf".to_owned(), 4),
        ("abdfg".to_owned(), 5),
        ("abdefg".to_owned(), 6),
        ("acf".to_owned(), 7),
        ("abcdefg".to_owned(), 8),
        ("abcdfg".to_owned(), 9)
    ].iter().cloned().collect();
}

fn deduce_impl(
    constraints: &VecDeque<Vec<i8>>,
    state: &mut VecDeque<i8>,
    candidates: &mut VecDeque<i8>,
) -> bool {
    lazy_static! {
        static ref CORRECT: Vec<Vec<i8>> = [
            "cf", "acf", "bcdf", "acdeg", "acdfg", "abdfg", "abcefg", "abdefg", "abcdfg", "abcdefg"
        ]
        .iter()
        .map(|s| s.chars().map(|c| CHAR_TO_DIGIT[&c]).sorted())
        .map(|v| Vec::from_iter(v))
        .collect();
    }

    // Permute original values using current state while masking out values for which no permutation has been defined
    let generated: FnvHashSet<Vec<i8>> = CORRECT
        .iter()
        .map(|m| {
            m.iter()
                .map(|&x| {
                    for i in 0..state.len() {
                        if x == i as i8 {
                            return state[i as usize];
                        }
                    }
                    -1
                })
                .collect()
        })
        .collect();

    let check_constraints = |g: &Vec<i8>| {
        for c in constraints.iter().filter(|x| x.len() == g.len()) {
            if c.len() == g.len() {
                if g.iter().all(|x| *x == -1 || c.contains(x)) {
                    return true;
                }
            }
        }

        false
    };

    if !generated.iter().all(check_constraints) {
        return false;
    }

    if candidates.is_empty() {
        return true;
    }

    // Check subproblems
    let n_candidates = candidates.len();
    for _ in 0..n_candidates {
        let trial = unsafe { candidates.pop_front().unwrap_unchecked() };

        state.push_back(trial as i8);
        if deduce_impl(constraints, state, candidates) {
            return true;
        }
        state.pop_back();

        candidates.push_back(trial);
    }

    // Dead end
    false
}

fn deduce(constraints: &VecDeque<Vec<i8>>) -> Vec<i8> {
    // Set up internal state
    let mut state = VecDeque::new();
    let mut candidates = (0..7).into_iter().collect();
    deduce_impl(constraints, &mut state, &mut candidates);

    // Invert permutation table
    let mut table = vec![0; state.len()];
    for i in 0..state.len() {
        table[state[i] as usize] = i as i8;
    }

    table
}

fn decode(permutation_table: &Vec<i8>, mut msg: Vec<i8>) -> Vec<i8> {
    for m in msg.iter_mut() {
        *m = permutation_table[*m as usize];
    }

    msg
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [problem_input]", args[0]);
        std::process::exit(-1);
    }

    let data = std::fs::read_to_string(&args[1]).expect("Unable to open input file");
    let mini_problems: Vec<(VecDeque<Vec<i8>>, VecDeque<Vec<i8>>)> = data
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut parts = s.split('|');
            let process_next = |x: &mut std::str::Split<char>| {
                x.next()
                    .expect("Unable to read problem")
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.chars().map(|c| CHAR_TO_DIGIT[&c]).sorted().collect())
                    .collect()
            };
            let input = process_next(&mut parts);
            let msg = process_next(&mut parts);
            (input, msg)
        })
        .collect();

    // Deduce permutation table from the given miniproblem and apply it to the validation output
    let solutions: Vec<Vec<u8>> = mini_problems
        .into_iter()
        .map(|(problem, msg)| {
            let permutation_table = deduce(&problem);

            msg.into_iter()
                .map(|m| decode(&permutation_table, m))
                .map(|m| m.into_iter().map(|c| DIGIT_TO_CHAR[&c]).sorted().collect())
                .map(|s: String| STR_TO_SEGMENT_VALUE[&s])
                .collect()
        })
        .collect();

    // Count numbers corresponding to segment codes with unique lengths
    let answer_one = solutions
        .iter()
        .flatten()
        .filter(|&&x| x == 1 || x == 4 || x == 7 || x == 8)
        .count();

    // Combine digits from the segment display as base-10 and then sum all of the rows
    let answer_two = solutions.iter().fold(0_usize, |acc, v| {
        let mut value = 0;
        for i in 0..v.len() {
            let power = v.len() - i - 1;
            value += 10_usize.pow(power as u32) * v[i] as usize;
        }
        acc + value
    });
    println!("{}", answer_one);
    println!("{}", answer_two);
}
