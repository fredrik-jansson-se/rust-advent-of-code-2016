use astar::*;
// use permutohedron::LexicalPermutation;
use permutohedron::heap_recursive;
use std::fs;
use std::ops::Add;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
enum Room {
    Wall,
    Open,
}

pub fn run() {
    let file = fs::read_to_string("day24.txt").unwrap();
    let w = parse(&file);
    println!("day24-1: {}", run_1(w.clone()));
    println!("day24-2: {}", run_2(w.clone()));
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Coord {
    pub x: isize,
    pub y: isize,
}

impl Coord {
    fn new(x: isize, y:isize) -> Coord {
        Coord {
            x: x,
            y: y,
        }
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Coord {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone)]
struct World {
    w: Vec<Vec<Room>>,
    start: Coord,
    goals: Vec<Coord>,
}

impl World {
    fn is_valid(&self, x: isize, y: isize) -> bool {
        x >= 0 && (x as usize) < self.w[0].len() &&
            y >= 0 && (y as usize) < self.w.len() &&
            self.w[y as usize][x as usize] == Room::Open
    }
}

impl AStar<Coord> for World {
    fn heuristic_cost_estimate(&self, from: &Coord, goal: &Coord) -> f32 {
        self.distance_between(from, goal)
    }

    fn distance_between(&self, from: &Coord, to: &Coord) -> f32 {
        let dx = (to.x - from.x) as f32;
        let dy = (to.y - from.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }

    fn neighbors(&self, n: &Coord) -> Vec<Coord> {
        let mut r = vec!{};

        r.push(n.clone() + Coord::new(0, 1));
        r.push(n.clone() + Coord::new(0, -1));
        r.push(n.clone() + Coord::new(-1, 0));
        r.push(n.clone() + Coord::new(1, 0));

        r.into_iter().filter(|c| self.is_valid(c.x, c.y)).collect()
    }
}

fn parse(s: &str) -> World {
    let mut w = vec!{};
    let mut start = Coord::new(0,0);
    let mut goals = vec!{};
    for (y, row) in s.lines().enumerate() {
        let mut v = vec!{};

        for (x, c) in row.chars().enumerate() {
            match c {
                '#' => v.push(Room::Wall),
                '.' => v.push(Room::Open),
                tc => {
                    let value: i32 = format!("{}", tc).parse().unwrap();
                    if 0 == value {
                        start = Coord::new(x as isize, y as isize);
                    }
                    else {
                        goals.push(Coord::new(x as isize, y as isize));
                    }
                    v.push(Room::Open);
                }
            }
        }

        w.push(v);
    }
    World { w: w, start: start, goals: goals }
}

fn way_len(w: &World, goals: &Vec<Coord>, lookup: &mut HashMap<(Coord, Coord), usize> ) -> usize {
    let mut cur = w.start.clone();
    let mut len = 0;
   
    for g in goals.iter() {
        let lu : Option<usize> = lookup.get(&(cur.clone(), g.clone())).map(|v| *v);
        len += match lu {
            Some(v) => v,
            None => {
                let s = w.solve(&cur, g).unwrap();
                // println!("{:?} ===> {:?}", cur, g);
                // println!("s = {:?}", s);
                // Remove 1 since the solution includes the start;
                let l = s.len();
                lookup.insert((cur.clone(), g.clone()), l);
                l
            }
        };
        cur = g.clone();
    }
    len
}

fn run_1(mut w: World) -> usize {
    let mut m = usize::max_value();
    let mut lu = HashMap::new();
    let mut perms = Vec::new();

    heap_recursive(&mut w.goals, |p| {
        perms.push(p.to_vec());
    });

    for goals in perms {
        m = usize::min(way_len(&w, &goals, &mut lu), m);
    }
    m
}

fn run_2(mut w: World) -> usize {
    let mut m = usize::max_value();
    let mut lu = HashMap::new();
    let mut perms = Vec::new();

    heap_recursive(&mut w.goals, |p| {
        perms.push(p.to_vec());
    });

    for mut goals in perms {
        // println!("{:?}", w.goals);
        goals.push(w.start.clone());
        m = usize::min(way_len(&w, &goals, &mut lu), m);
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day24_1() {
        let sworld = r"###########
#0.1.....2#
#.#######.#
#4.......3#
###########";
        let w = parse(&sworld);
        assert_eq!(5, w.w.len());
        assert_eq!(11, w.w[0].len());
        assert_eq!(Coord::new(1,1), w.start);
        assert_eq!(4, w.goals.len());

        assert_eq!(14, run_1(w));
    }
}
