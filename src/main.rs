use aoc2021::*;
use std::io::Cursor;
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buf).unwrap();

    let part = std::env::args().nth(1).expect("missing part");
    let res: Box<dyn std::fmt::Display> = match part.as_str() {
        // "24a" => Box::new(d24_lobby::part1(&buf)),
        // "24b" => Box::new(d24_lobby::part2(&buf)),
        _ => panic!("Bad part {}", part),
    };
    println!("{}", res);
}
