use tokio::io::{AsyncRead, BufReader, AsyncBufReadExt};
use tokio_stream::StreamExt;

pub async fn part1<R: AsyncRead + std::marker::Unpin>(input: BufReader<R>) -> u64 {
    let lines = tokio_stream::wrappers::LinesStream::new(AsyncBufReadExt::lines(input));
    let mut depths = StreamExt::map(lines, |x| x.unwrap().parse::<u32>().unwrap());
    let prev_depth = depths.next().await;
    let mut prev_depth = match prev_depth {
        None => return 0,
        Some(x) => x,
    };
    let mut increases = 0;
    while let Some(depth) = depths.next().await {
        //println!("Handling {} to {}", prev_depth, depth);
        if depth > prev_depth {
            increases += 1
        }
        prev_depth = depth;
    }
    increases
}

#[cfg(test)]
mod testing {
    use super::*;

    #[tokio::test(flavor="multi_thread")]
    async fn test_example() {
        let input = BufReader::new("199
200
208
210
200
207
240
269
260
263".as_bytes());
        assert_eq!(part1(input).await, 7);
    }
}