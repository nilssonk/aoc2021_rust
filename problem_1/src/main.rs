fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [problem_input]", args[0]);
        std::process::exit(-1);
    }

    let data = std::fs::read_to_string(&args[1]).expect("Unable to open input file");

    let measurements: Vec<u16> = 
        data
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u16>().expect("Parsing failed"))
        .collect();

    let mut answer_one = 0;
    {
        let mut prev = u16::MAX;
        for elem in &measurements {
            if *elem > prev {
                answer_one += 1;
            }
            prev = *elem;
        }
    }

    let mut answer_two = 0;
    {
        let mut prev = u16::MAX;
        for i in 0..(measurements.len() - 2) {
            let a = measurements[i];
            let b = measurements[i+1];
            let c = measurements[i+2];
            let sum = a + b + c;
            if sum > prev {
                answer_two += 1;
            }
            prev = sum;
        }
    }
    
    println!("{}", answer_one);
    println!("{}", answer_two);
}
