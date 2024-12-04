use std::{fs, time::Instant};

use eyre::Result;

mod solutions;

fn main() -> Result<()> {
    let input = fs::read_to_string("./src/input/day04.txt")?;

    let start = Instant::now();

    let solution = solutions::day04::part2(&input);

    let elapsed = start.elapsed();

    println!("{solution}");
    println!("Elapsed: {elapsed:?}");

    Ok(())
}
