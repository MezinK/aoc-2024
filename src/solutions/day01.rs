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

    let mut sum = 0;

    for l in &left {
        let mut seen = 0;
        for r in &right {
            if l == r {
                seen += 1;
            }
        }
        sum += seen * l;
    }

    sum
}
