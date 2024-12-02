fn get_all_levels(input: &str) -> impl Iterator<Item = Vec<i32>> + use<'_> {
    input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .collect()
    })
}

fn is_safe(levels: &[i32]) -> bool {
    levels.windows(2).all(|pair| {
        let asc_diff = pair[1] - pair[0];
        (asc_diff > 0 && asc_diff < 4)
    }) || levels.windows(2).all(|pair| {
        let desc_diff = pair[0] - pair[1];
        (desc_diff > 0 && desc_diff < 4)
    })
}

pub fn part1(input: &str) -> usize {
    get_all_levels(input)
        .filter(|levels| is_safe(levels))
        .count()
}

pub fn part2(input: &str) -> usize {
    get_all_levels(input)
        .filter(|levels| {
            is_safe(levels)
                || levels.iter().enumerate().any(|(idx, _)| {
                    let mut temp = levels.clone();
                    temp.remove(idx);
                    is_safe(&temp)
                })
        })
        .count()
}
