use std::{fs, time::Instant};

use eyre::Result;

mod solutions;

fn main() -> Result<()> {
    let input = fs::read_to_string("./src/input/day09.txt")?;

    let start: Instant = Instant::now();

    let solution = solutions::day09::part1(&input);

    let elapsed = start.elapsed();

    println!("{solution}");
    println!("Elapsed: {elapsed:?}");

    Ok(())
}
