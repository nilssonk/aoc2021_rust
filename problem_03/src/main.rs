fn solve_one(input: &[u16], width: usize) -> u32 {
    // Count bits per position
    let mut counts = vec![0_u16; width];
    for r in input {
        for (i, count) in counts.iter_mut().enumerate() {
            *count += (r & (1 << i) != 0) as u16;
        }
    }

    // Compile most common bits to gamma
    let predicate = |x| x as usize * 2 > input.len();
    let mut gamma = 0_u32;
    for (i, count) in counts.iter().enumerate() {
        let bit_value = predicate(*count) as u32;
        gamma |= bit_value << i;
    }

    // Invert gamma to get least common bits
    let invert = |x| {
        let mut mask = 0_u32;
        for i in 0..width {
            mask |= 1 << i;
        }
        x ^ mask
    };
    let epsilon = invert(gamma);

    gamma * epsilon
}

fn solve_two_impl(input: &[u16], width: usize, predicate: impl Fn(usize, usize) -> bool) -> u16 {
    let mut results = std::collections::VecDeque::with_capacity(input.len());
    for &r in input {
        results.push_back(r);
    }

    // Iteratively refine result set using predicate
    let mut i = width as isize - 1;
    while results.len() > 1 && i >= 0 {
        let mut count = 0;
        for q in &results {
            count += (q & (1 << i) != 0) as usize;
        }

        // Retain only if the ith bit matches the condition
        let condition = predicate(count, results.len());
        results.retain(|&x| (x & (1 << i) != 0) == condition);

        i -= 1;
    }

    *results
        .front()
        .expect("No element satisfying predicate could be found")
}

fn solve_two(input: &[u16], width: usize) -> u32 {
    let o_2 = solve_two_impl(input, width, |a, b| 2 * a >= b) as u32;
    let co_2 = solve_two_impl(input, width, |a, b| 2 * a < b) as u32;

    o_2 * co_2
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
    let width = first.len();

    let entries: Vec<u16> = rows
        .map(|s| u16::from_str_radix(s, 2).expect("Unable to parse integer"))
        .collect();

    let answer_one = solve_one(&entries, width);
    let answer_two = solve_two(&entries, width);

    println!("{}", answer_one);
    println!("{}", answer_two);
}
