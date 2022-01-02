use fnv::FnvHashSet;

type Integer = i32;
type Vec2 = (Integer, Integer);

fn to_index(input: &[u8]) -> usize {
    let mut result = 0_usize;
    for (i, &x) in input.iter().enumerate() {
        result |= (x as usize) << (input.len() - i - 1);
    }

    result
}

fn print(input: &[u8]) {
    for &x in input.iter() {
        if x > 0 {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!();
}

#[derive(Clone)]
struct PointSet {
    data: FnvHashSet<Vec2>,
    x_min: Integer,
    x_max: Integer,
    y_min: Integer,
    y_max: Integer,
}

impl PointSet {
    fn from_old_limits(other: &Self) -> Self {
        Self {
            data: FnvHashSet::default(),
            x_min: other.x_min,
            x_max: other.x_max,
            y_min: other.y_min,
            y_max: other.y_max,
        }
    }

    fn new(input: FnvHashSet<Vec2>) -> Self {
        let mut x_min = Integer::MAX;
        let mut x_max = Integer::MIN;
        let mut y_min = Integer::MAX;
        let mut y_max = Integer::MIN;
        for &(x, y) in input.iter() {
            x_min = std::cmp::min(x_min, x);
            x_max = std::cmp::max(x_max, x);
            y_min = std::cmp::min(y_min, y);
            y_max = std::cmp::max(y_max, y);
        }
        Self {
            data: input,
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    fn insert(&mut self, a @ (x, y): Vec2) {
        self.x_min = std::cmp::min(self.x_min, x - 1);
        self.x_max = std::cmp::max(self.x_max, x + 1);
        self.y_min = std::cmp::min(self.y_min, y - 1);
        self.y_max = std::cmp::max(self.y_max, y + 1);

        self.data.insert(a);
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn print(&self) {
        for y in self.y_min..(self.y_max + 1) {
            for x in self.x_min..(self.x_max + 1) {
                if self.data.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn mutate(orig: Self, mask: &[u8]) -> Self {
        let mut next = PointSet::from_old_limits(&orig);
        for y in orig.y_min..(orig.y_max + 1) {
            for x in orig.x_min..(orig.x_max + 1) {
                let mut index_recipe = Vec::new();
                for i in -1..2 {
                    let index_y = y + i;
                    for j in -1..2 {
                        let index_x = x + j;
                        index_recipe
                            .push(orig.data.get(&(index_x, index_y)).map(|_| 1).unwrap_or(0));
                    }
                }
                let mask_index = to_index(&index_recipe);
                let new_value = mask[mask_index];
                if new_value > 0 {
                    next.insert((x, y));
                }
            }
        }

        next
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [problem_input]", args[0]);
        std::process::exit(-1);
    }

    let data = std::fs::read_to_string(&args[1]).expect("Unable to open input file");
    let mut parts = data.split("\n\n").filter(|s| !s.is_empty());

    let char_to_u8 = |c| match c {
        '#' => 1_u8,
        '.' => 0_u8,
        _ => panic!(),
    };

    let mask: Vec<u8> = parts
        .next()
        .unwrap()
        .chars()
        .filter(|&c| c != '\n')
        .map(char_to_u8)
        .collect();

    let input_lines = parts.next().unwrap().split('\n').filter(|s| !s.is_empty());

    let mut input = FnvHashSet::default();
    for (y, l) in input_lines.enumerate() {
        for (x, _) in l.chars().enumerate().filter(|&(_, c)| c == '#') {
            input.insert((x as Integer, y as Integer));
        }
    }

    let mut points = PointSet::new(input);
    //points.print();
    for _ in 0..2 {
        points = PointSet::mutate(points, &mask);
        //points.print();
    }
    let answer_one = points.len();
    println!("{}", answer_one);

    for _ in 0..48 {
        points = PointSet::mutate(points, &mask);
    }
    let answer_two = points.len();
    println!("{}", answer_two);
    //print(&mask);
}
