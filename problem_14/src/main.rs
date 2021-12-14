#![feature(iter_intersperse)]

use fnv::FnvHashMap;

type MutationMap<'a> = FnvHashMap<&'a str, (String, String)>;

fn parse_input(input: &String) -> (Vec<char>, MutationMap) {
    let mut parts = input.split("\n\n");

    let template: Vec<char> = parts
        .next()
        .expect("Unable to read template")
        .chars()
        .collect();

    let rules: MutationMap = parts
        .next()
        .expect("Unable to read rules")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut subparts = s.split(" -> ");
            let from = subparts.next().expect("Corrupt rule encountered");
            let to = subparts
                .next()
                .expect("Corrupt rule encountered")
                .trim()
                .chars()
                .next()
                .unwrap();
            let triplet: String = from.chars().intersperse(to).collect();
            let first = triplet[0..2].to_owned();
            let second = triplet[1..3].to_owned();

            (from, (first, second))
        })
        .collect();

    (template, rules)
}

fn count_difference(input: &FnvHashMap<char, isize>) -> isize {
    let mut mincount = isize::MAX;
    let mut maxcount = isize::MIN;
    for (_, &count) in input.iter() {
        if count < mincount {
            mincount = count;
        }
        if count > maxcount {
            maxcount = count;
        }
    }

    maxcount - mincount
}

fn mutate(rules: &MutationMap, input: &Vec<char>, iterations: usize) -> isize {
    // Make sure all valid fragments have entries
    let mut fragment_counts = FnvHashMap::default();
    for (&from, _) in rules.iter() {
        fragment_counts.entry(from).or_insert(0_isize);
    }

    // Count fragments in input
    for win in input.windows(2) {
        let candidate: String = win.iter().collect();
        let e = unsafe {
            fragment_counts
                .get_mut(candidate.as_str())
                .unwrap_unchecked()
        };
        *e += 1;
    }

    let mut letter_counts = FnvHashMap::default();

    // Mutate
    for _ in 0..iterations {
        // Save fragment counter updates since we cannot lookup an element while already borrowing another
        let mut counter_updates = Vec::new();
        for (k, count) in fragment_counts.iter_mut().filter(|(_, count)| **count > 0) {
            if let Some((ref a, ref b)) = rules.get(k) {
                // Split pair as per rule
                counter_updates.push((a.as_str(), *count));
                counter_updates.push((b.as_str(), *count));

                // Compensate letter count for shared letters
                let shared_letter = unsafe { b.chars().next().unwrap_unchecked() };
                let e = letter_counts.entry(shared_letter).or_insert(0);
                *e -= *count;

                *count = 0;
            }
        }

        // Update fragment counters
        for (k, count) in counter_updates {
            let v = unsafe { fragment_counts.get_mut(k).unwrap_unchecked() };
            *v += count;
        }
    }

    // Count letters
    for (k, v) in fragment_counts {
        let mut char_iter = k.chars();
        let first = unsafe { char_iter.next().unwrap_unchecked() };
        let second = unsafe { char_iter.next().unwrap_unchecked() };

        let a = letter_counts.entry(first).or_insert(0);
        *a += v;
        let b = letter_counts.entry(second).or_insert(0);
        *b += v;
    }

    count_difference(&letter_counts)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [problem_input]", args[0]);
        std::process::exit(-1);
    }

    let data = std::fs::read_to_string(&args[1]).expect("Unable to open input file");

    let (template, rules) = parse_input(&data);

    let answer_one = mutate(&rules, &template, 10);
    let answer_two = mutate(&rules, &template, 40);

    println!("{}", answer_one);
    println!("{}", answer_two);
}
