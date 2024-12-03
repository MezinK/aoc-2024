use regex::Regex;

pub fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input).fold(0, |sum, caps| {
        let l = caps[1].parse::<u32>().unwrap();
        let r = caps[2].parse::<u32>().unwrap();

        sum + l * r
    })
}

pub fn part2(input: &str) -> u32 {
    let re = Regex::new(r"(?P<do>do\(\))|(?P<dont>don't\(\))|mul\((?P<l>\d{1,3}),(?P<r>\d{1,3})\)")
        .unwrap();

    re.captures_iter(input)
        .fold((0, true), |(sum, enabled), caps| {
            let is_enabled = match (caps.name("do").is_some(), caps.name("dont").is_some()) {
                (true, _) => true,
                (_, true) => false,
                _ => enabled,
            };

            if !is_enabled {
                return (sum, is_enabled);
            }

            return match (caps.name("l"), caps.name("r")) {
                (Some(l_match), Some(r_match)) => {
                    let l = l_match.as_str().parse::<u32>().unwrap();
                    let r = r_match.as_str().parse::<u32>().unwrap();

                    return (sum + l * r, is_enabled);
                }
                _ => (sum, is_enabled),
            };
        })
        .0
}
