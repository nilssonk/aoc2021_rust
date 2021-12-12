use fnv::FnvHashMap;
use smallvec::SmallVec;
use stable_vec::StableVec;
use std::collections::hash_map::Entry::*;
use std::iter::FromIterator;

enum NodeType {
    SingleVisit,
    MultiVisit,
}

struct Node {
    pub node_type: NodeType,
    pub connections: SmallVec<[u8; 4]>,
}

#[derive(Default)]
struct NodeStorage {
    pub data: StableVec<Node>,
    pub index: FnvHashMap<String, u8>,
}

impl NodeStorage {
    fn connect(&mut self, name_a: &str, name_b: &str) {
        let index_a = self.index[name_a];
        let index_b = self.index[name_b];

        // Make sure the start node is only visited once
        if name_b != "start" {
            let a = unsafe { self.data.get_unchecked_mut(index_a as usize) };
            a.connections.push(index_b);
        }
        if name_a != "start" {
            let b = unsafe { self.data.get_unchecked_mut(index_b as usize) };
            b.connections.push(index_a);
        }
    }

    fn insert(&mut self, name: String) {
        if let Vacant(e) = self.index.entry(name) {
            let key = e.key();
            let first = key.chars().next().unwrap();

            let node_type = if first.is_lowercase() {
                NodeType::SingleVisit
            } else {
                NodeType::MultiVisit
            };

            let index = self.data.push(Node {
                node_type: node_type,
                connections: SmallVec::new(),
            });
            e.insert(index as u8);
        }
    }

    fn paths(&self, start: u8, end: u8, may_double: bool) -> usize {
        let allowed_counts = FnvHashMap::from_iter(self.data.iter().filter_map(|(i, n)| {
            if let NodeType::SingleVisit = n.node_type {
                Some((i as u8, 1))
            } else {
                None
            }
        }));

        let mut explore_queue: SmallVec<[_; 32]> = SmallVec::new();
        explore_queue.push((start, allowed_counts, may_double));

        let mut result = 0;
        while let Some((this, mut allowed, mut allow_double)) = explore_queue.pop() {
            if this == end {
                result += 1;
                continue;
            }

            let node = unsafe { self.data.get_unchecked(this as usize) };

            match node.node_type {
                NodeType::SingleVisit => {
                    if let Occupied(ref mut e) = allowed.entry(this) {
                        let count = e.get_mut();

                        if *count == 0 {
                            if allow_double {
                                allow_double = false;
                            } else {
                                continue;
                            }
                        } else {
                            *count = 0;
                        }
                    }
                }
                _ => (),
            }

            for &c in node.connections.iter() {
                explore_queue.push((c, allowed.clone(), allow_double));
            }
        }

        result
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [problem_input]", args[0]);
        std::process::exit(-1);
    }

    let data = std::fs::read_to_string(&args[1]).expect("Unable to open input file");
    let rows = data.split('\n').filter(|s| !s.is_empty());

    let mut storage = NodeStorage::default();
    for s in rows {
        let mut halves = s.split('-');
        let a = halves.next().expect("Could not read first node");
        let b = halves.next().expect("Could not read second node");
        storage.insert(a.to_owned());
        storage.insert(b.to_owned());
        storage.connect(a, b);
    }

    let start = storage.index["start"];
    let end = storage.index["end"];
    let answer_one = storage.paths(start, end, false);
    let answer_two = storage.paths(start, end, true);

    println!("{}", answer_one);
    println!("{}", answer_two);
}
