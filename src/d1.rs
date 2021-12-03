use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};
use tokio_stream::StreamExt;

/// Look at a sliding window of size `N`, looking at the sum of each window, and
/// returning the number of sums that were an increase over the previous sum.
pub async fn count_increases<
    R: AsyncRead + std::marker::Unpin,
    const N: usize,
>(
    input: BufReader<R>,
) -> u64 {
    let lines =
        tokio_stream::wrappers::LinesStream::new(AsyncBufReadExt::lines(input));
    let mut depths =
        StreamExt::map(lines, |x| x.unwrap().parse::<u32>().unwrap());

    // Read the first `N` into an array.
    let mut prevs = [0u32; N];
    let mut last_sum = 0u32;
    for prev in prevs.iter_mut() {
        let d = depths.next().await.unwrap();
        *prev = d;
        last_sum += d;
    }

    // For each subsequent value we compute the new sliding sum and compare to
    // the previous one.
    let mut increases = 0;
    let mut oldest_idx = 0;
    while let Some(depth) = depths.next().await {
        let next_sum = last_sum - prevs[oldest_idx] + depth;
        if next_sum > last_sum {
            increases += 1;
        }
        prevs[oldest_idx] = depth;
        oldest_idx = (oldest_idx + 1) % N;
        last_sum = next_sum;
    }
    increases
}

pub async fn part1<R: AsyncRead + std::marker::Unpin>(
    input: BufReader<R>,
) -> u64 {
    count_increases::<_, 1>(input).await
}

pub async fn part2<R: AsyncRead + std::marker::Unpin>(
    input: BufReader<R>,
) -> u64 {
    count_increases::<_, 3>(input).await
}

#[cfg(test)]
mod testing {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_example() {
        let s = "199
200
208
210
200
207
240
269
260
263";
        assert_eq!(part1(BufReader::new(s.as_bytes())).await, 7);
        assert_eq!(part2(BufReader::new(s.as_bytes())).await, 5);
    }
}
