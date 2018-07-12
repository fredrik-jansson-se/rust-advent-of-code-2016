use std::fs;
use regex::Regex;

pub fn run() {
    println!("day22-1: {}", run_1());
}

#[derive(Debug)]
struct Node {
    x: usize,
    y: usize,
    total: usize,
    used: usize,
    avail: usize,
}

impl Node {
    fn is_viable(&self, other: &Node) -> bool {
        return self.used > 0 &&
            self.used <= other.avail;
    }
}

fn s2i(s: &str) -> usize {
    usize::from_str_radix(s, 10).unwrap()
}

fn parse_node(line: &str) -> Option<Node> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"/dev/grid/node-x(\d+)-y(\d+)\s*(\d+)T\s*(\d+)T\s*(\d+)T"#).unwrap();
    }

    match RE.captures(line) {
        None => None,
        Some(c) => {
            Some(Node {
                x: s2i(&c[1]),
                y: s2i(&c[2]),
                total: s2i(&c[3]),
                used: s2i(&c[4]),
                avail: s2i(&c[5]),
            })
        }
    }
}

fn run_1() -> usize {
    let file = fs::read_to_string("day22.txt").unwrap();
    let nodes : Vec<Node> = file.lines().filter_map(parse_node).collect();
    // println!("# nodes: {}", nodes.len());
    // println!("nodes: {:?}", nodes);
    let mut valid = 0;
    for i in 0..(nodes.len() - 1) {
        for j in i+1..nodes.len() {
            if nodes[i].is_viable(&nodes[j]) ||
                nodes[j].is_viable(&nodes[i]) {
                    valid += 1;
                }
        }
    }
    valid
}
