use std::str::FromStr;
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};
use tokio_stream::StreamExt;

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();
        let cmd_token = tokens.next().unwrap();
        let x = tokens.next().unwrap().parse().unwrap();
        Ok(match cmd_token {
            "forward" => Command::Forward(x),
            "down" => Command::Down(x),
            "up" => Command::Up(x),
            _ => panic!("Unrecognized: {}", cmd_token),
        })
    }
}

struct Position {
    horizontal: u32,
    depth: u32,
}

impl Position {
    fn process_command(&mut self, cmd: Command) {
        match cmd {
            Command::Forward(x) => self.horizontal += x,
            Command::Down(x) => self.depth += x,
            Command::Up(x) => self.depth -= x,
        }
    }
}

pub async fn part1<R: AsyncRead + std::marker::Unpin>(
    input: BufReader<R>,
) -> u32 {
    let mut p = Position {
        horizontal: 0,
        depth: 0,
    };
    let lines =
        tokio_stream::wrappers::LinesStream::new(AsyncBufReadExt::lines(input));
    let mut commands =
        StreamExt::map(lines, |x| x.unwrap().parse::<Command>().unwrap());
    while let Some(cmd) = commands.next().await {
        p.process_command(cmd)
    }
    p.horizontal * p.depth
}

#[cfg(test)]
mod testing {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_example() {
        let s = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(part1(BufReader::new(s.as_bytes())).await, 150);
    }
}
