use regex::Regex;
use std::ops::Add;

#[derive(Default)]
struct PositionOne {
    x: u32,
    y: u32,
}

#[derive(Default)]
struct PositionTwo {
    x: u32,
    y: u32,
    aim: u32,
}

enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl Add<&Direction> for PositionOne {
    type Output = Self;
    fn add(mut self, other: &Direction) -> Self {
        use Direction::*;
        match other {
            Forward(d) => self.x += d,
            Up(d) => self.y -= d,
            Down(d) => self.y += d,
        }
        self
    }
}

impl Add<&Direction> for PositionTwo {
    type Output = Self;
    fn add(mut self, other: &Direction) -> Self {
        use Direction::*;
        match other {
            Forward(d) => {
                self.x += d;
                self.y += self.aim * d;
            }
            Up(d) => self.aim -= d,
            Down(d) => self.aim += d,
        }
        self
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [problem_input]", args[0]);
        std::process::exit(-1);
    }

    let data = std::fs::read_to_string(&args[1]).expect("Unable to open input file");
    let commands = data.split('\n').filter(|s| !s.is_empty());

    let re = Regex::new(r"(forward|up|down) (\d+)").unwrap();

    let (pos_one, pos_two) = commands.fold(
        (PositionOne::default(), PositionTwo::default()),
        |(acc_one, acc_two), s| {
            let cap = re.captures(&s).expect("Regex capture failed");

            let dist = cap[2].parse::<u32>().expect("Integer parsing failed");
            use Direction::*;
            let dir = match &cap[1] {
                "forward" => Forward(dist),
                "up" => Up(dist),
                "down" => Down(dist),
                _ => panic!("Invalid direction encountered"),
            };

            (acc_one + &dir, acc_two + &dir)
        },
    );
    let answer_one = pos_one.x * pos_one.y;
    let answer_two = pos_two.y * pos_two.x;

    println!("{}", answer_one);
    println!("{}", answer_two);
}
