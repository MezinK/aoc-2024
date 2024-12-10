use std::{fs, time::Instant};

use eyre::Result;

mod solutions;

fn main() -> Result<()> {
    let input = fs::read_to_string("./src/input/day10.txt")?;

    let start: Instant = Instant::now();

    let solution = solutions::day10::part2(&input);

    let elapsed = start.elapsed();

    println!("{solution}");
    println!("Elapsed: {elapsed:?}");

    Ok(())
}
