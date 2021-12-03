use aoc2021::*;

#[tokio::main]
async fn main() {
    // Pass `stdin` as an *async stream* to solutions. This is extremely overkill, but
    // passing it as a stream allows us to avoid keeping the entire input in memory
    // for some solutions.
    //
    // That it's an *async* stream is... not very useful when this is roughly
    // the only I/O in the whole program. In principle though we could plug
    // these solutions into something like a web "solutions server" and
    // efficiently handle many concurrent requests with a small number of
    // threads.
    let stdin = tokio::io::BufReader::new(tokio::io::stdin());

    // The only command-line argument specifies which solution to run.
    let part = std::env::args().nth(1).expect("missing part");
    match part.as_str() {
         "1a" => println!("{}", d1::part1(stdin).await),
         "1b" => println!("{}", d1::part2(stdin).await),
        _ => panic!("Bad part {}", part),
    };
}