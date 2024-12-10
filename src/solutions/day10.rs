use std::{
    collections::HashSet,
    ops::{Add, Index},
};

pub fn part1(input: &str) -> usize {
    let map = Map::new(input);
    let mut stack = Vec::new();
    let mut set = HashSet::new();

    let mut sum = 0;

    for (idx, byte) in map.bytes.iter().enumerate() {
        if *byte != b'0' {
            continue;
        }

        let start = Pos::from_index(idx, (map.w + 1) as usize);
        stack.push(start);
        set.clear();

        while let Some(pos) = stack.pop() {
            let height = map[pos];
            if height == b'9' {
                set.insert(pos);
                continue;
            }

            for direction in DIRECTIONS {
                let next = pos + direction;
                if !map.contains(next) {
                    continue;
                }

                if map[next] == height + 1 {
                    stack.push(next);
                }
            }
        }

        sum += set.len();
    }

    sum
}

pub fn part2(input: &str) -> usize {
    let map = Map::new(input);
    let mut stack = Vec::new();

    let mut sum = 0;

    for (idx, byte) in map.bytes.iter().enumerate() {
        if *byte != b'0' {
            continue;
        }

        let start = Pos::from_index(idx, (map.w + 1) as usize);
        stack.push(start);

        while let Some(pos) = stack.pop() {
            let height = map[pos];
            if height == b'9' {
                sum += 1;
                continue;
            }

            for direction in DIRECTIONS {
                let next = pos + direction;
                if !map.contains(next) {
                    continue;
                }

                if map[next] == height + 1 {
                    stack.push(next);
                }
            }
        }
    }

    sum
}

struct Map<'a> {
    bytes: &'a [u8],
    w: i16,
    h: i16,
}

impl<'a> Map<'a> {
    fn new(input: &'a str) -> Self {
        let bytes = input.as_bytes();
        let w = memchr::memchr(b'\n', bytes).unwrap();
        let h = bytes.len() / w;

        Self {
            bytes,
            w: w as i16,
            h: h as i16,
        }
    }

    fn contains(&self, pos: Pos) -> bool {
        (0..self.w).contains(&pos.x) && (0..self.h).contains(&pos.y)
    }
}

impl<'a> Index<Pos> for Map<'a> {
    type Output = u8;
    fn index(&self, index: Pos) -> &Self::Output {
        &self.bytes[index.y as usize * (self.w as usize + 1) + index.x as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i16,
    y: i16,
}

impl Pos {
    const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    fn from_index(idx: usize, w: usize) -> Self {
        Self::new((idx % w) as i16, (idx / w) as i16)
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

const DIRECTIONS: [Pos; 4] = [
    Pos::new(-1, 0),
    Pos::new(0, 1),
    Pos::new(1, 0),
    Pos::new(0, -1),
];
