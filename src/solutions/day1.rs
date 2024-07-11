use anyhow::Result;

use crate::utils::get_input;

const NUMS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn nums(line: &[u8], i: usize) -> Option<usize> {
    line[i]
        .is_ascii_digit()
        .then_some((line[i] - b'0') as usize)
        .or(NUMS
            .iter()
            .enumerate()
            .find(|(_, name)| line[i..].starts_with(name))
            .map(|(num, _)| num + 1))
}

pub fn day_1(is_p2: bool) -> Result<String> {
    let res = get_input(1)?;
    if !is_p2 {
        let res = res
            .lines()
            .map(|line| {
                ((line
                    .as_bytes()
                    .iter()
                    .find(|b| b.is_ascii_digit())
                    .unwrap_or(&0)
                    - b'0')
                    * 10
                    + line
                        .as_bytes()
                        .iter()
                        .rev()
                        .find(|b| b.is_ascii_digit())
                        .unwrap_or(&0)
                    - b'0') as usize
            })
            .sum::<usize>()
            .to_string();
        Ok(res)
    } else {
        let res = res
            .lines()
            .map(|line| {
                (0..line.len())
                    .find_map(|i| nums(line.as_bytes(), i))
                    .unwrap_or(0)
                    * 10
                    + (0..line.len())
                        .rev()
                        .find_map(|i| nums(line.as_bytes(), i))
                        .unwrap_or(0)
            })
            .sum::<usize>()
            .to_string();
        Ok(res)
    }
}
