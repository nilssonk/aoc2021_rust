type ResultCache = std::collections::HashMap<(u16, u8), usize>;

fn spawn(days: u16, until_spawn: u8, cache: &mut ResultCache) -> usize {
    if let Some(&cached) = cache.get(&(days, until_spawn)) {
        return cached;
    }

    let result = match (days, until_spawn) {
        (0, _) => 1,
        (_, 0) => spawn(days - 1, 6, cache) + spawn(days - 1, 8, cache),
        _ => spawn(days - 1, until_spawn - 1, cache),
    };

    cache.insert((days, until_spawn), result);

    result
}

fn simulate(input: &Vec<u8>, days: u16, cache: &mut ResultCache) -> usize {
    input.iter().map(|&x| spawn(days, x, cache)).sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [problem_input]", args[0]);
        std::process::exit(-1);
    }

    let data = std::fs::read_to_string(&args[1]).expect("Unable to open input file");

    let fishies: Vec<u8> = data
        .lines()
        .flat_map(|s| s.split(','))
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u8>().expect("Unable to parse integer"))
        .collect();

    let mut cache = ResultCache::new();
    let answer_one = simulate(&fishies, 80, &mut cache);
    let answer_two = simulate(&fishies, 256, &mut cache);

    println!("{}", answer_one);
    println!("{}", answer_two);
}
