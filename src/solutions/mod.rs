use anyhow::Result;
use day1::day_1;
use day2::day_2;
use day3::day_3;
use day4::day_4;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn solution(day: u8, is_p2: bool) -> Result<String> {
    match day {
        1 => day_1(is_p2),
        2 => day_2(is_p2),
        3 => day_3(is_p2),
        4 => day_4(is_p2),
        _ => Ok("No solution yet".to_string()),
    }
}
