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
    aim: u32,
}

impl Position {
    fn process_command_p1(&mut self, cmd: Command) {
        match cmd {
            Command::Forward(x) => self.horizontal += x,
            Command::Down(x) => self.depth += x,
            Command::Up(x) => self.depth -= x,
        }
    }

    fn process_command_p2(&mut self, cmd: Command) {
        match cmd {
            Command::Forward(x) => {
                self.horizontal += x;
                self.depth += x * self.aim;
            }
            Command::Down(x) => self.aim += x,
            Command::Up(x) => self.aim -= x,
        }
    }
}

async fn drive<
    R: AsyncRead + std::marker::Unpin,
    F: Fn(&mut Position, Command),
>(
    input: BufReader<R>,
    drive: F,
) -> u32 {
    let mut p = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };
    let lines =
        tokio_stream::wrappers::LinesStream::new(AsyncBufReadExt::lines(input));
    let mut commands =
        StreamExt::map(lines, |x| x.unwrap().parse::<Command>().unwrap());
    while let Some(cmd) = commands.next().await {
        drive(&mut p, cmd);
    }
    p.horizontal * p.depth
}

pub async fn part1<R: AsyncRead + std::marker::Unpin>(
    input: BufReader<R>,
) -> u32 {
    drive(input, |p, c| p.process_command_p1(c)).await
}

pub async fn part2<R: AsyncRead + std::marker::Unpin>(
    input: BufReader<R>,
) -> u32 {
    drive(input, |p, c| p.process_command_p2(c)).await
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
        assert_eq!(part2(BufReader::new(s.as_bytes())).await, 900);
    }
}
