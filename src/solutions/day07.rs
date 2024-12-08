use itertools::Itertools;
use std::ops::{Add, Mul};

pub fn part1(input: &str) -> u64 {
    solve(input, |eq| eq.check::<Part1>())
}

pub fn part2(input: &str) -> u64 {
    solve(input, |eq| eq.check::<Part2>())
}

fn solve(input: &str, check: fn(&Equation) -> bool) -> u64 {
    input
        .lines()
        .map(|line| {
            let eq = Equation::parse(line);

            if check(&eq) {
                eq.value
            } else {
                0
            }
        })
        .sum()
}

struct Equation<'a> {
    value: u64,
    nums: &'a [u64],
}

impl<'a> Equation<'a> {
    fn parse(line: &str) -> Self {
        let (unparsed_value, unparsed_nums) = line.split_once(":").unwrap();

        let nums = unparsed_nums
            .trim()
            .split_ascii_whitespace()
            .map(str::parse::<u64>)
            .map(Result::unwrap)
            .collect_vec();

        Self {
            value: unparsed_value.parse::<u64>().unwrap(),
            nums: nums.leak(),
        }
    }

    fn check<C: Check>(&self) -> bool {
        let [curr, rest @ ..] = self.nums else {
            return false;
        };

        C::r(self.value, *curr, rest)
    }
}

struct Part1;

impl Check for Part1 {
    const OPERATIONS: &[fn(u64, u64) -> u64] = &[u64::add, u64::mul];
}

struct Part2;

impl Check for Part2 {
    const OPERATIONS: &[fn(u64, u64) -> u64] = &[u64::add, u64::mul, concat];
}

fn concat(a: u64, b: u64) -> u64 {
    (a.to_string() + &b.to_string()).parse::<u64>().unwrap()
}

trait Check {
    const OPERATIONS: &[fn(u64, u64) -> u64];
    const N: usize = Self::OPERATIONS.len();

    fn check(target: u64, curr: u64, next: u64, rest: &[u64]) -> bool {
        Self::OPERATIONS
            .iter()
            .any(|op| Self::r(target, op(curr, next), rest))
    }

    fn r(target: u64, curr: u64, rest: &[u64]) -> bool {
        let [next, rest @ ..] = rest else {
            return curr == target;
        };

        if curr > target {
            return false;
        }

        Self::check(target, curr, *next, rest)
    }
}
