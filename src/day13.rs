use std::ops::Add;
use astar::*;

pub fn run() {
    println!("day13-1: {}", run_1(1364, 31, 39));
    println!("day13-2: {}", run_2(1364));
}

fn run_1(fav_num: isize, dest_x: isize, dest_y: isize) -> usize {
    let ac = CoordAStar { fav_num: fav_num };
    let solution = ac.solve(&Coord::new(1,1), &Coord::new(dest_x, dest_y)).unwrap();

    solution.len()
}

fn run_2(fav_num: isize) -> usize {
    let start = Coord::new(1,1);
    let ac = CoordAStar { fav_num: fav_num };
    let mut cnt = 0;
    for x in 0..55 {
        for y in 0..55 {
            if is_open(x,y, fav_num) {
                let goal = Coord::new(x,y);
                let solution = ac.solve(&start, &goal);
                match solution {
                    Some(s) => if s.len() <= 50 { cnt+= 1}
                    None => ()
                }
            }
        }
    }
    cnt
}

fn is_open(x: isize, y: isize, fav_num: isize) -> bool {
    // (x+y)^2 + y
    let mut f = x*x + 3*x + 2*x*y +y + y*y + fav_num;

    let mut cnt = 0;

    while f > 0 {
        if f & 1 != 0 {
            cnt += 1;
        }
        f >>= 1;
    }

    cnt % 2 == 0
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

struct CoordAStar {
    fav_num: isize
}

impl AStar<Coord> for CoordAStar {
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


        // r.into_iter().filter(|c| c.x >= 0 && c.y >=0).collect()
        let fav_num = self.fav_num;
        // r.into_iter().filter(|c| c.x >= 0 && c.y >=0 && is_open(c.x, c.y, fav_num).collect()
        r.into_iter().filter(|c| is_valid(&c, fav_num)).collect()
    }
}

fn is_valid(c: &Coord, fav_num: isize) -> bool {
    c.x >= 0 && c.y >= 0 && is_open(c.x, c.y, fav_num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13_is_open() {
        assert_eq!(true, is_open(0, 0, 10));
        assert_eq!(false, is_open(1, 0, 10));
        assert_eq!(true, is_open(2, 0, 10));
        assert_eq!(false, is_open(0, 2, 10));
        assert_eq!(true, is_open(3, 3, 10));
        assert_eq!(false, is_open(8, 6, 10));
    }

    #[test]
    fn day13_day_1() {
        assert_eq!(11, run_1(10, 7, 4));
    }
}
