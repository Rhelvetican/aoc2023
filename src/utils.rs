use std::{fmt::Display, fs::read_to_string, io::stdin};

use anyhow::Result;

pub fn get_input(day: u8) -> Result<String> {
    let path = format!("./input/day_{}.txt", day);
    let res = read_to_string(path).unwrap_or(String::from(""));
    Ok(res)
}

pub fn input<M: Display>(msg: M) -> Result<String> {
    println!("{}", msg);
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    Ok(buf)
}
