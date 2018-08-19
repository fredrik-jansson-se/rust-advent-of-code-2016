extern crate astar;
#[macro_use] extern crate lazy_static;
extern crate permutohedron;
extern crate regex;

use std::env;

mod day11;
mod day13;
mod day22;
mod day24;

fn main() {
    let mut a = env::args();
    a.next();

    let day = match a.next() {
        Some(s) => usize::from_str_radix(&s, 10).unwrap(),
        None => 0,
    };

    match day {
        11 => day11::run(),
        13 => day13::run(),
        22 => day22::run(),
        24 => day24::run(),
        _ => ()
    }
}
