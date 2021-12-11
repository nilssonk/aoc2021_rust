use smallvec::smallvec;
use smallvec::SmallVec;

use std::collections::HashSet;
use std::collections::VecDeque;

type Vec2 = (isize, isize);

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn at<'a, T>(data: &'a [T], (width, height): Vec2, (x, y): Vec2) -> Option<&'a T> {
    if x < 0 || x >= width || y < 0 || y >= height {
        return None;
    }

    let index = y * width + x;
    unsafe { Some(data.get_unchecked(index as usize)) }
}

unsafe fn at_unchecked<'a, T>(data: &'a [T], (width, _): Vec2, (x, y): Vec2) -> &'a T {
    let index = y * width + x;
    data.get_unchecked(index as usize)
}

fn explore_basin(data: &Vec<u8>, dim @ (width, height): Vec2, start_p: Vec2) -> HashSet<Vec2> {
    use Direction::*;

    let outset: (Vec2, SmallVec<[_; 4]>) = (start_p, smallvec![Left, Right, Up, Down]);
    let mut frontier = VecDeque::from([outset]);

    let mut visited = HashSet::new();
    while let Some((p @ (x, y), dirs)) = frontier.pop_front() {
        let &this = unsafe { at_unchecked(data, dim, p) };
        if this >= 9 || !visited.insert(p) {
            continue;
        }
        for d in dirs {
            let next @ ((u, v), _) = match d {
                Left => ((x - 1, y), smallvec![Left, Up, Down]),
                Right => ((x + 1, y), smallvec![Right, Up, Down]),
                Up => ((x, y - 1), smallvec![Left, Right, Up]),
                Down => ((x, y + 1), smallvec![Left, Right, Down]),
            };
            if u >= 0 && u < width && v >= 0 && v < height {
                frontier.push_back(next);
            }
        }
    }

    visited
}

fn find_minima(height_map: &Vec<u8>, dim @ (width, height): Vec2) -> Vec<Vec2> {
    let is_minimum = |(x, y)| {
        let this = unsafe { at_unchecked(&height_map, dim, (x, y)) };

        let neighbors = [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)];
        for p in neighbors {
            if let Some(adj) = at(&height_map, dim, p) {
                if this >= adj {
                    return false;
                }
            }
        }

        true
    };

    let mut minima = Vec::new();
    let mut y = 0;
    while y < height {
        let mut x = 0;
        while x < width {
            let p = (x, y);
            if is_minimum(p) {
                minima.push(p);

                // Skip next in line since it is by definition not a minimum
                x += 1;
            }

            x += 1;
        }

        y += 1;
    }

    minima
}

fn solve_one(height_map: &Vec<u8>, dim: Vec2, input: &Vec<Vec2>) -> usize {
    // Sum of x+1 over all minima x
    input.iter().fold(0, |acc, &p| {
        let &val = unsafe { at_unchecked(&height_map, dim, p) };
        acc + val as usize + 1
    })
}

fn solve_two(height_map: &Vec<u8>, dim: Vec2, input: &Vec<Vec2>) -> usize {
    // Find unique basins
    let mut basins: Vec<HashSet<Vec2>> = input
        .iter()
        .map(|&p| explore_basin(&height_map, dim, p))
        .collect();
    basins.dedup();

    // Take the product of the 3 largest
    basins.sort_by(|a, b| b.len().cmp(&a.len()));

    basins.iter().map(|x| x.len()).take(3).product::<usize>()
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

    let height_map: Vec<u8> = rows
        .flat_map(|s| s.chars())
        .map(|c| c as u8 - '0' as u8)
        .collect();

    let height = height_map.len() as isize / width;
    let dim = (width, height);

    let minima = find_minima(&height_map, dim);

    let answer_one = solve_one(&height_map, dim, &minima);
    let answer_two = solve_two(&height_map, dim, &minima);

    println!("{}", answer_one);
    println!("{}", answer_two);
}
