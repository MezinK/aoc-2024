use std::{borrow::Cow, collections::HashSet, ops::Add};

pub fn part1(input: &str) -> usize {
    let map = Map::new(input);
    let start = map.start();

    map.trace(start as i16).len()
}

pub fn part2(input: &str) -> usize {
    let map = Map::new(input);
    let start = map.start();

    let mut state = State::new(&map);

    map.trace(start as i16)
        .iter()
        .filter(|&i| {
            state.input.bytes.to_mut()[*i as usize] = b'#';
            let is_loop = state.is_loop(start as i16);
            state.input.bytes.to_mut()[*i as usize] = b'.';

            is_loop
        })
        .count()
}

#[derive(Clone)]
struct Map<'a> {
    bytes: Cow<'a, [u8]>,
    w: i16,
    h: i16,
}

impl<'a> Map<'a> {
    fn new(input: &'a str) -> Self {
        let bytes = input.as_bytes();
        let w = memchr::memchr(b'\n', bytes).unwrap() as i16 + 1;
        let h = (bytes.len() as i16 + 1) / w;

        Self {
            bytes: Cow::Borrowed(bytes),
            w,
            h,
        }
    }

    fn start(&self) -> usize {
        memchr::memchr(b'^', &self.bytes).unwrap()
    }

    fn contains(&self, pos: Pos) -> bool {
        (0..self.w - 1).contains(&pos.x) && (0..self.h).contains(&pos.y)
    }

    fn trace(&self, start: i16) -> HashSet<i16> {
        let mut seen = HashSet::new();
        seen.insert(start);

        let mut dir = Direction::UP;
        let mut curr = Pos::new(start % self.w, start / self.w);

        loop {
            let next = curr + dir;

            if !self.contains(next) {
                return seen;
            }

            let idx = next.to_idx(self.w);

            if self.bytes[idx as usize] == b'#' {
                dir.rotate();
            } else {
                seen.insert(idx);
                curr = next;
            }
        }
    }
}

const DIRECTION_OFFSETS: [Pos; 4] = [
    Pos::new(0, -1),
    Pos::new(1, 0),
    Pos::new(0, 1),
    Pos::new(-1, 0),
];

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Direction(u8);

impl Direction {
    const UP: Self = Self(0);

    fn rotate(&mut self) {
        self.0 += 1;
        self.0 %= 4;
    }
}

impl Add<Direction> for Pos {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        Self {
            x: self.x + DIRECTION_OFFSETS[rhs.0 as usize].x,
            y: self.y + DIRECTION_OFFSETS[rhs.0 as usize].y,
        }
    }
}

#[derive(Clone, Copy)]
struct Pos {
    x: i16,
    y: i16,
}

impl Pos {
    const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    fn to_idx(self, w: i16) -> i16 {
        self.y * w + self.x
    }
}

struct State<'a> {
    input: Map<'a>,
    seen: HashSet<(i16, Direction)>,
}

impl<'a> State<'a> {
    fn new(input: &Map<'a>) -> Self {
        Self {
            input: input.to_owned(),
            seen: HashSet::new(),
        }
    }

    fn is_loop(&mut self, start: i16) -> bool {
        let mut dir = Direction::UP;
        let mut curr = Pos::new(start % self.input.w, start / self.input.w);

        self.seen.clear();
        self.seen.insert((start, dir));

        loop {
            let next = curr + dir;

            if !self.input.contains(next) {
                break;
            }

            let idx = next.to_idx(self.input.w);

            if self.input.bytes[idx as usize] == b'#' {
                dir.rotate();
            } else {
                if !self.seen.insert((idx, dir)) {
                    return true;
                }

                curr = next;
            }
        }

        false
    }
}
