#[macro_use]
extern crate lazy_static;

const BITS: usize = 4;

#[derive(Copy, Clone)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

struct Operator {
    op_type: OperatorType,
    subpackets: Vec<Packet>,
}

impl Operator {
    fn eval(&self) -> usize {
        use OperatorType::*;
        let iter = self.subpackets.iter().map(|x| x.eval());
        match self.op_type {
            Sum => iter.sum(),
            Product => iter.product(),
            Minimum => iter.min().unwrap(),
            Maximum => iter.max().unwrap(),
            GreaterThan => (self.subpackets[0].eval() > self.subpackets[1].eval()) as usize,
            LessThan => (self.subpackets[0].eval() < self.subpackets[1].eval()) as usize,
            EqualTo => (self.subpackets[0].eval() == self.subpackets[1].eval()) as usize,
        }
    }
}

enum PacketType {
    Lit(usize),
    Op(Operator),
}

struct Packet {
    version: u8,
    packet_type: PacketType,
}

impl Packet {
    fn sum_version(&self) -> usize {
        use PacketType::*;
        self.version as usize
            + match &self.packet_type {
                Lit(_) => 0,
                Op(o) => o.subpackets.iter().map(|x| x.sum_version()).sum(),
            }
    }
    fn eval(&self) -> usize {
        use PacketType::*;
        match &self.packet_type {
            Lit(value) => *value,
            Op(o) => o.eval(),
        }
    }
}

fn concat_usize(input: &[u8]) -> usize {
    let bit = |x, n| (x >> n) & 0x1;
    let bit_at = |i| bit(input[i / BITS], i % BITS);

    let mut result = 0_usize;
    let length = input.len() * BITS;
    for i in 0..length {
        result |= (bit_at(i) as usize) << i;
    }

    result
}

fn read_bits(input: &[u8], bit_pos: &mut usize, n_bits: usize) -> Option<Vec<u8>> {
    let mut current_pos = *bit_pos;

    if (current_pos + n_bits) / BITS >= input.len() {
        return None;
    }

    let bit = |x, n| (x >> n) & 0x1;
    let bit_at = |i| bit(input[i / BITS], BITS - i % BITS - 1);

    let result_length = (n_bits + (BITS - 1)) / BITS;
    let mut result = vec![0_u8; result_length];

    let top_bits = n_bits % BITS;
    for i in 0..top_bits {
        let read_pos = current_pos + i;
        let input_bit = bit_at(read_pos);

        let out_byte = result_length - 1;
        let out_bit = top_bits - i - 1;

        result[out_byte] |= input_bit << out_bit;
    }
    current_pos += top_bits;

    let remaining = n_bits - top_bits;
    for i in 0..remaining {
        let read_pos = current_pos + i;
        let input_bit = bit_at(read_pos);

        let out_byte = result_length - i / BITS - 2;
        let out_bit = BITS - i % BITS - 1;
        result[out_byte] |= input_bit << out_bit;
    }
    current_pos += remaining;

    *bit_pos = current_pos;

    Some(result)
}

fn operator_type(x: u8) -> OperatorType {
    use OperatorType::*;

    lazy_static! {
        static ref TYPE_LOOKUP: Vec<OperatorType> = vec![
            Sum,
            Product,
            Minimum,
            Maximum,
            Maximum, // Dummy
            GreaterThan,
            LessThan,
            EqualTo];
    }

    TYPE_LOOKUP[x as usize]
}

fn parse_operator(input: &[u8], version: u8, op_type: u8, position: &mut usize) -> Option<Packet> {
    let length_type_id = read_bits(input, position, 1)?;

    let limit_type = if length_type_id[0] == 0 {
        let total_length_bits = read_bits(input, position, 15)?;
        Limit::Pos(*position + concat_usize(&total_length_bits))
    } else {
        let max_packet_bits = read_bits(input, position, 11)?;
        Limit::Num(concat_usize(&max_packet_bits))
    };

    Some(Packet {
        version: version,
        packet_type: PacketType::Op(Operator {
            op_type: operator_type(op_type),
            subpackets: parse(input, position, limit_type),
        }),
    })
}

enum Limit {
    Pos(usize),
    Num(usize),
}

impl Limit {
    fn predicate(&self, position: usize, num: usize) -> bool {
        match self {
            Limit::Pos(p) => position < *p,
            Limit::Num(n) => num < *n,
        }
    }
}

fn parse(input: &[u8], position: &mut usize, max: Limit) -> Vec<Packet> {
    let mut result = Vec::new();

    let parse_one = |pos: &mut usize| {
        let v = read_bits(&input, pos, 3)?;
        let t = read_bits(&input, pos, 3)?;

        match t[0] {
            4 => {
                let mut literal = Vec::new();
                loop {
                    let tmp = read_bits(&input, pos, 5)?;
                    literal.push(tmp[0]);
                    if tmp[1] == 0 {
                        break;
                    }
                }
                literal.reverse();
                Some(Packet {
                    version: v[0],
                    packet_type: PacketType::Lit(concat_usize(&literal)),
                })
            }
            op_type => parse_operator(&input, v[0], op_type, pos),
        }
    };

    while max.predicate(*position, result.len()) {
        match parse_one(position) {
            Some(p) => result.push(p),
            None => break,
        }
    }

    result
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [problem_input]", args[0]);
        std::process::exit(-1);
    }

    let parse_hex = |char| {
        let b = char as u8;
        if char >= 'a' {
            b - 'a' as u8 + 10
        } else if char >= 'A' {
            b - 'A' as u8 + 10
        } else {
            b - '0' as u8
        }
    };

    let data = std::fs::read_to_string(&args[1]).expect("Unable to open input file");
    let stream: Vec<u8> = data.chars().map(parse_hex).collect();

    let mut position = 0;
    let packets = parse(&stream, &mut position, Limit::Pos(stream.len() * BITS));

    let answer_one = packets.iter().map(|p| p.sum_version()).sum::<usize>();
    println!("{}", answer_one);

    let answer_two = packets.iter().next().expect("Must have one packet").eval();
    println!("{}", answer_two);
}
