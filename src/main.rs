
use aoc2021::*;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let part = std::env::args().nth(1).expect("missing part");
    match part.as_str() {
         "1a" => println!("{}", d1::part1(stdin).await),
         "1b" => println!("{}", d1::part2(stdin).await),
        _ => panic!("Bad part {}", part),
    };
}