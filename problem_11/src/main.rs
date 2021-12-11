fn simulate(input: &mut [u8], width: isize, height: isize) -> usize {
    // Use highest bit as flash indicator
    let has_flashed = |x| x & (1 << 7) != 0;
    let flashed = |x| x | 1 << 7;
    let value = |x| x & !(1_u8 << 7);

    let index = |x, y| (y * width + x) as usize;

    for e in input.iter_mut() {
        *e += 1;
    }

    loop {
        let mut no_flashes = true;

        for y in 0..height {
            let top = std::cmp::max(0, y - 1);
            let bottom = std::cmp::min(height, y + 2);

            for x in 0..width {
                let this = unsafe { input.get_unchecked_mut(index(x, y)) };
                if value(*this) > 9 && !has_flashed(*this) {
                    // Mark as flashed
                    *this = flashed(*this);
                    no_flashes = false;

                    // Trigger neighbors
                    let left = std::cmp::max(0, x - 1);
                    let right = std::cmp::min(width, x + 2);
                    for i in top..bottom {
                        for j in left..right {
                            let val = unsafe { input.get_unchecked_mut(index(j, i)) };
                            *val += 1;
                        }
                    }
                }
            }
        }

        if no_flashes {
            break;
        }
    }

    // Count flashes and reset energy levels
    let mut flashes = 0;
    for e in input.iter_mut() {
        if has_flashed(*e) {
            flashes += 1;
            *e = 0;
        }
    }

    flashes
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
    let width = first.len() as isize;
    let mut energy_map: Vec<u8> = rows
        .flat_map(|s| s.chars())
        .map(|c| c as u8 - '0' as u8)
        .collect();

    let height = energy_map.len() as isize / width;

    let mut answer_one = 0;
    for _ in 0..100 {
        answer_one += simulate(&mut energy_map, width, height);
    }

    let mut answer_two = 101;
    while simulate(&mut energy_map, width, height) != energy_map.len() {
        answer_two += 1;
    }

    println!("{}", answer_one);
    println!("{}", answer_two);
}
