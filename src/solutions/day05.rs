use std::collections::HashMap;

use itertools::Itertools;

fn load_stack_rules(rules: &[&str]) -> HashMap<u8, Vec<u8>> {
    let mut stack_rules: HashMap<u8, Vec<u8>> = HashMap::new();

    let rule_pairs = rules
        .iter()
        .map(|l| {
            l.split_once("|")
                .map(|n| (n.0.parse::<u8>().unwrap(), n.1.parse::<u8>().unwrap()))
                .unwrap()
        })
        .collect_vec();

    for rule_pair in rule_pairs {
        stack_rules
            .entry(rule_pair.0)
            .and_modify(|e| e.push(rule_pair.1))
            .or_insert(vec![rule_pair.1]);
    }

    stack_rules
}

pub fn part1(input: &str) -> u32 {
    let binding = input.lines().collect_vec();
    let mut iter = binding.split(|i| i == &"");

    let unparsed_rules = iter.next().unwrap();
    let updates = iter.next().unwrap();

    let rules = load_stack_rules(unparsed_rules);

    updates
        .iter()
        .filter_map(|update| {
            let iter = update.split(",").map(str::parse::<u8>).map(Result::unwrap);
            let mut state: Vec<u8> = Vec::new();

            state.extend(iter);

            rules
                .iter()
                .all(|(num, rule)| {
                    let Some(num_idx) = memchr::memchr(*num, &state) else {
                        return true;
                    };

                    rule.iter().all(|&rule_num| {
                        memchr::memchr(rule_num, &state).map_or(true, |rule_idx| num_idx < rule_idx)
                    })
                })
                .then_some(state[state.len() / 2] as u32)
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let binding = input.lines().collect_vec();
    let mut iter = binding.split(|i| i == &"");

    let stack_rules = iter.next().unwrap();
    let updates = iter.next().unwrap();

    let rules = load_stack_rules(stack_rules);

    updates
        .iter()
        .filter_map(|update| {
            let iter = update.split(",").map(str::parse::<u8>).map(Result::unwrap);
            let mut state: Vec<u8> = Vec::new();
            state.extend(iter);

            let mut sorted = false;
            let mut iters = 0;

            while !sorted {
                sorted = true;
                iters += 1;

                for (num, rule) in rules.iter() {
                    let Some(num_idx) = memchr::memchr(*num, &state) else {
                        continue;
                    };

                    for r in rule {
                        if let Some(rule_idx) = memchr::memchr(*r, &state) {
                            if num_idx > rule_idx {
                                sorted = false;
                                state.swap(num_idx, rule_idx);
                            }
                        }
                    }
                }
            }

            (iters > 1).then_some(state[state.len() / 2] as u32)
        })
        .sum()
}
