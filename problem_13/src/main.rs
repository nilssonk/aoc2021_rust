use fnv::FnvHashSet;
use regex::Regex;

type Vec2 = (i32, i32);

#[derive(Copy, Clone)]
enum Fold {
    Vertical(i32),
    Horizontal(i32),
}

fn parse_input(input: &str) -> (FnvHashSet<Vec2>, Vec<Fold>) {
    let mut points = FnvHashSet::default();
    let mut folds = Vec::new();

    // Capture (x,y) or "fold along coord_name=coord_value"
    let re =
        Regex::new(r"((?P<x>\d+),(?P<y>\d+)|fold along (?P<coord_name>.)=(?P<coord_value>\d+))")
            .unwrap();

    let rows = input.split('\n').filter(|s| !s.is_empty());
    for r in rows {
        let caps = re.captures(r).expect("Unable to capture regex");
        if let Some(x_cap) = caps.name("x") {
            let x_str = x_cap.as_str();
            let y_str = caps.name("y").unwrap().as_str();

            let parse_int = |s: &str| s.parse::<i32>().expect("Unable to parse integer");
            let x = parse_int(x_str);
            let y = parse_int(y_str);

            points.insert((x, y));
        } else {
            let name = caps.name("coord_name").unwrap().as_str();
            let value = caps
                .name("coord_value")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .expect("Unable to parse integer");

            if name == "x" {
                folds.push(Fold::Horizontal(value));
            } else {
                folds.push(Fold::Vertical(value));
            }
        }
    }

    (points, folds)
}

fn fold(points: &mut FnvHashSet<Vec2>, (w, h): Vec2, f: Fold) -> Vec2 {
    let (reflected, new_dim): (Vec<Vec2>, Vec2) = match f {
        Fold::Horizontal(new_w) => (
            points
                .iter()
                .filter(|&&(x, y)| x >= new_w && x <= w && y >= 0 && y <= h)
                .map(|&(x, y)| (2 * new_w - x, y))
                .collect(),
            (new_w, h),
        ),
        Fold::Vertical(new_h) => (
            points
                .iter()
                .filter(|&&(x, y)| x >= 0 && x <= w && y >= new_h && y <= h)
                .map(|&(x, y)| (x, 2 * new_h - y))
                .collect(),
            (w, new_h),
        ),
    };

    for r in reflected {
        points.insert(r);
    }

    points.retain(|&(x, y)| x >= 0 && x <= new_dim.0 && y >= 0 && y <= new_dim.1);

    new_dim
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [problem_input]", args[0]);
        std::process::exit(-1);
    }

    let data = std::fs::read_to_string(&args[1]).expect("Unable to open input file");

    let (mut points, folds) = parse_input(&data);

    let width = points.iter().map(|&(x, _)| x).max().unwrap();
    let height = points.iter().map(|&(_, y)| y).max().unwrap();
    let mut dim = (width, height);

    let mut fold_iter = folds.iter();

    // Solve part one
    dim = fold(&mut points, dim, *fold_iter.next().unwrap());
    let answer_one = points.len();
    println!("{}", answer_one);

    // Solve part two
    for &f in fold_iter {
        dim = fold(&mut points, dim, f);
    }

    // Print answer two
    for i in 0..dim.1 {
        for j in 0..dim.0 {
            if points.contains(&(j, i)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
