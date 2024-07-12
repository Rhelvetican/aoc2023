use anyhow::{anyhow, Result};
use solutions::solution;
use utils::input;

mod solutions;
mod utils;

fn main() -> Result<()> {
    let day = input("Enter the day: ")?.trim().parse::<u8>()?;
    let is_p2 = input("Is it part 2? (y/n/t/f): ")?
        .trim()
        .to_ascii_lowercase();

    let is_p2 = match is_p2.as_str() {
        "y" | "yes" | "t" | "true" => true,
        "n" | "no" | "f" | "false" => false,
        _ => return Err(anyhow!("Invalid input")),
    };
    let sol = solution(day, is_p2)?;
    println!("{}", sol);
    Ok(())
}
