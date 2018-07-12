extern crate regex;
#[macro_use] extern crate lazy_static;

use std::env;

mod day11;
mod day22;

fn main() {
    let mut a = env::args();
    a.next();

    let day = match a.next() {
        Some(s) => usize::from_str_radix(&s, 10).unwrap(),
        None => 0,
    };

    match day {
        11 => day11::run(),
        22 => day22::run(),
        _ => ()
    }
}
