fn minimize(x0: i64, loss: impl Fn(i64) -> i64) -> i64 {
    let derivatives = |x| {
        let forward = loss(x + 1);
        let back = loss(x - 1);
        let this = loss(x);
        (forward - this, back - this)
    };

    let mut x = x0;

    loop {
        let (forward, back) = derivatives(x);
        if forward > 0 && back > 0 {
            break;
        }

        x += (back - forward).signum() as i64;
    }

    x
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [problem_input]", args[0]);
        std::process::exit(-1);
    }

    let data = std::fs::read_to_string(&args[1]).expect("Unable to open input file");

    let positions: Vec<u16> = data
        .lines()
        .flat_map(|s| s.split(','))
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u16>().expect("Unable to parse integer"))
        .collect();

    let sum = positions.iter().fold(0_i64, |acc, x| acc + *x as i64);
    let avg_x = sum / positions.len() as i64;

    let l1_loss = |x: i64| {
        positions
            .iter()
            .fold(0_i64, |acc, &n| acc + (n as i64 - x).abs())
    };

    let polynomial_loss = |x: i64| {
        positions.iter().fold(0_i64, |acc, &n| {
            let d = (n as i64 - x).abs();
            // Use the sum formula and try to avoid overflow
            acc + if d % 2 == 0 {
                (d / 2) * (d + 1)
            } else {
                ((d + 1) / 2) * d
            }
        })
    };

    // Note: Minimizing with respect to L1 produces the median
    let min_one = minimize(avg_x, &l1_loss);
    let min_two = minimize(avg_x, &polynomial_loss);

    // Assume the loss functions are convex and have no local optima
    let answer_one = l1_loss(min_one);
    let answer_two = polynomial_loss(min_two);

    println!("{}", answer_one);
    println!("{}", answer_two);
}
