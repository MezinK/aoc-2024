use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

pub fn part1(input: &str) -> usize {
    let map = Map::parse(input);

    map.get_antinodes(|antinodes, x, y| {
        let diff = x - y;

        let dx = x + diff;
        let dy = y - diff;

        if map.contains(dx) {
            antinodes.insert(dx);
        }

        if map.contains(dy) {
            antinodes.insert(dy);
        }
    })
}

pub fn part2(input: &str) -> usize {
    let map = Map::parse(input);

    map.get_antinodes(|antinodes, mut x, mut y| {
        let diff = x - y;

        while map.contains(x) {
            antinodes.insert(x);
            x = x + diff;
        }

        while map.contains(y) {
            antinodes.insert(y);
            y = y - diff;
        }
    })
}

struct Map {
    w: i16,
    h: i16,
    antennas: HashMap<u8, Vec<Pos>>,
}

impl Map {
    fn parse(input: &str) -> Map {
        let mut antennas: HashMap<u8, Vec<Pos>> = HashMap::new();

        let mut find_antennas = |y, line| {
            for (x, byte) in str::bytes(line).enumerate() {
                if byte != b'.' {
                    antennas
                        .entry(byte)
                        .and_modify(|v| v.push(Pos::new(x, y)))
                        .or_insert(vec![Pos::new(x, y)]);
                }
            }
        };

        let mut lines = input.lines().enumerate();

        let (_, line) = lines.next().unwrap();
        find_antennas(0, line);

        let w = line.len() as i16;
        let mut h: i16 = 1;

        for (y, line) in lines {
            h += 1;
            find_antennas(y, line)
        }

        Self { w, h, antennas }
    }

    fn get_antinodes(&self, func: impl Fn(&mut Antinodes, Pos, Pos)) -> usize {
        let mut antinodes = HashSet::new();

        for signals in self.antennas.values() {
            for i in 0..signals.len() {
                for j in i + 1..signals.len() {
                    func(&mut antinodes, signals[i], signals[j]);
                }
            }
        }

        antinodes.len()
    }

    fn contains(&self, pos: Pos) -> bool {
        (0..self.w).contains(&pos.x) && (0..self.h).contains(&pos.y)
    }
}

type Antinodes = HashSet<Pos>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i16,
    y: i16,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Self {
            x: x as i16,
            y: y as i16,
        }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
