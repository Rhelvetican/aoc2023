use anyhow::{anyhow, Result};

use crate::utils::get_input;

pub fn day_2(is_p2: bool) -> Result<String> {
    let input = get_input(2)?;
    let mut res = 0;

    if !is_p2 {
        for (id, game) in input.lines().enumerate() {
            let mut rgb = [0, 0, 0];
            let game = game.split(':').nth(1).unwrap_or("");
            for color in game.split(|c| c == ',' || c == ';') {
                let color = color.trim();
                let parts = color.split_whitespace().collect::<Vec<&str>>();
                let amount = parts[0].parse::<u32>()?;
                let color = parts[1];
                match color {
                    "red" => rgb[0] = amount.max(rgb[0]),
                    "green" => rgb[1] = amount.max(rgb[1]),
                    "blue" => rgb[2] = amount.max(rgb[2]),
                    _ => return Err(anyhow!("Invalid color")),
                }
            }
            if rgb[0] <= 12 && rgb[1] <= 13 && rgb[2] <= 14 {
                res += id + 1;
            }
        }
    } else {
        for game in input.lines() {
            let mut rgb = [0, 0, 0];
            let game = game.split(':').nth(1).unwrap_or("");
            for color in game.split(|c| c == ',' || c == ';') {
                let color = color.trim();
                let parts = color.split_whitespace().collect::<Vec<&str>>();
                let amount = parts[0].parse::<u32>()?;
                let color = parts[1];
                match color {
                    "red" => rgb[0] = amount.max(rgb[0]),
                    "green" => rgb[1] = amount.max(rgb[1]),
                    "blue" => rgb[2] = amount.max(rgb[2]),
                    _ => return Err(anyhow!("Invalid color")),
                }
            }
            res += rgb.iter().product::<u32>() as usize;
        }
    }

    Ok(res.to_string())
}
