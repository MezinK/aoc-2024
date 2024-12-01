use std::collections::HashMap;

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let mut res = line
                .split_whitespace()
                .map(str::parse::<i32>)
                .map(Result::unwrap);

            (res.next().unwrap(), res.next().unwrap())
        })
        .unzip()
}

pub fn part1(input: &str) -> i32 {
    let (mut left, mut right) = parse(input);

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right)
        .map(|pair| (pair.0 - pair.1).abs())
        .sum::<i32>()
}

pub fn part2(input: &str) -> i32 {
    let (left, right) = parse(input);

    let mut seen = HashMap::<i32, i32>::new();

    for r in right {
        *seen.entry(r).or_insert(0) += 1;
    }

    left.iter()
        .filter_map(|l| Some(seen.get(l)? * l))
        .sum::<i32>()
}
