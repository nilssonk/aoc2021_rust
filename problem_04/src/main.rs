use arrayvec::ArrayVec;
use fnv::FnvHashMap;

struct Square {
    data: u8,
}

impl Square {
    pub fn new(number: u8) -> Self {
        Self {
            data: number & 0x7F,
        }
    }

    pub fn mark(&mut self) {
        self.data |= 0x80;
    }

    pub fn number(&self) -> u8 {
        self.data & 0x7F
    }

    pub fn is_marked(&self) -> bool {
        (self.data & 0x80) != 0
    }
}

struct BingoBoard {
    pub rows: ArrayVec<ArrayVec<Square, 5>, 5>,
    numbers: FnvHashMap<u8, usize>,
}

impl BingoBoard {
    pub fn new() -> Self {
        Self {
            rows: ArrayVec::new(),
            numbers: FnvHashMap::default(),
        }
    }

    pub fn add_row(&mut self, row: ArrayVec<u8, 5>) {
        self.rows
            .push(row.iter().map(|&n| Square::new(n)).collect());
        let index = self.rows.len() - 1;
        for &n in &row {
            self.numbers.insert(n, index);
        }
    }

    fn mark(&mut self, row_index: usize, number: u8) -> bool {
        let row = &mut self.rows[row_index];

        let col_index = row
            .iter_mut()
            .position(|x| x.number() == number)
            .expect("Corrupt BingoBoard encountered");
        let square = &mut row[col_index];
        square.mark();

        // Row check
        if row.iter().all(|x| x.is_marked()) {
            return true;
        }

        // Column check
        assert_eq!(self.rows.len(), 5);
        if self.rows.iter().all(|r| r[col_index].is_marked()) {
            return true;
        }

        false
    }

    pub fn draw(&mut self, number: u8) -> bool {
        if let Some(&index) = self.numbers.get(&number) {
            return self.mark(index, number);
        }

        false
    }
}

type BingoMap = FnvHashMap<usize, BingoBoard>;

fn parse_draws(data: String) -> Vec<u8> {
    data.split(',')
        .map(|s| s.parse::<u8>().expect("Unable to parse integer"))
        .collect()
}

fn parse_boards(data: String) -> BingoMap {
    let rows: Vec<&str> = data.split('\n').filter(|s| !s.is_empty()).collect();
    let mut i = 0;
    rows.chunks(5)
        .map(|chunk| {
            let mut board = BingoBoard::new();
            for row in chunk {
                let numbers = row
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<u8>().expect("Unable to parse integer"))
                    .collect();
                board.add_row(numbers);
            }
            i += 1;
            (i, board)
        })
        .collect()
}

fn do_draw(draw: u8, boards: &mut BingoMap) -> Option<usize> {
    for (&i, v) in boards {
        if v.draw(draw) {
            return Some(i);
        }
    }

    None
}

fn play(draws: &[u8], boards: &mut BingoMap) -> Option<(usize, u8)> {
    for &d in draws {
        if let Some(winning_index) = do_draw(d, boards) {
            return Some((winning_index, d));
        }
    }

    None
}

fn count_score(boards: &BingoMap, index: usize, draw: u8) -> u32 {
    let winner = &boards[&index];
    let sum_of_unmarked = winner
        .rows
        .iter()
        .flat_map(|x| x.iter())
        .filter(|&b| !b.is_marked())
        .fold(0_u32, |acc, x| acc + x.number() as u32);

    sum_of_unmarked * draw as u32
}

fn solve_one(draws: &[u8], boards: &mut BingoMap) -> u32 {
    let (winning_index, winning_draw) = play(draws, boards).expect("No winner found");
    count_score(boards, winning_index, winning_draw)
}

fn solve_two(draws: &[u8], mut boards: BingoMap) -> u32 {
    while boards.len() > 1 {
        let (w, _) = play(draws, &mut boards).expect("No winner found");
        boards.remove(&w);
    }

    let (winning_index, winning_draw) = play(draws, &mut boards).expect("No winner found");
    count_score(&boards, winning_index, winning_draw)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [problem_input]", args[0]);
        std::process::exit(-1);
    }

    let mut data = std::fs::read_to_string(&args[1]).expect("Unable to open input file");

    // Split input into draws and boards
    let split_index = data.find('\n').expect("Invalid input data");
    let boards_only = data.split_off(split_index);

    // Parse draws
    let draws = parse_draws(data);

    // Parse boards
    let mut boards = parse_boards(boards_only);

    // Solve
    let answer_one = solve_one(&draws, &mut boards);
    let answer_two = solve_two(&draws, boards);

    println!("{}", answer_one);
    println!("{}", answer_two);
}
