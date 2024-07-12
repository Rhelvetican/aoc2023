use std::collections::HashMap;

use anyhow::Result;

use crate::utils::get_input;

pub fn day_3(is_p2: bool) -> Result<String> {
    let input = get_input(3)?
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut ptsum = 0;
    let mut curnum = None;
    let mut curstart = None;

    if !is_p2 {
        for (r, line) in input.iter().enumerate() {
            for (c, ch) in line.iter().copied().enumerate() {
                let mut numeric = false;
                if let Some(n) = ch.to_digit(10) {
                    numeric = true;
                    curnum = Some(curnum.unwrap_or(0) * 10 + n);
                    if curstart.is_none() {
                        curstart = Some(c);
                    }
                }
                if !numeric || c == line.len() - 1 {
                    if let Some((start, n)) = curstart.zip(curnum) {
                        let end = if numeric { c } else { c - 1 };
                        let mut sym = None;
                        for i in r.saturating_sub(1)..=r.saturating_add(1) {
                            for j in start.saturating_sub(1)..=end.saturating_add(1) {
                                if let Some(&ch) = input.get(i).and_then(|l| l.get(j)) {
                                    if !ch.is_numeric() && ch != '.' {
                                        sym = Some(ch);
                                    }
                                }
                            }
                        }
                        if sym.is_some() {
                            ptsum += n;
                        }
                    }
                    curnum = None;
                    curstart = None;
                }
            }
        }
    } else {
        let mut gears = HashMap::new();
        for (r, line) in input.iter().enumerate() {
            for (c, ch) in line.iter().copied().enumerate() {
                let mut numeric = false;
                if let Some(n) = ch.to_digit(10) {
                    numeric = true;
                    curnum = Some(curnum.unwrap_or(0) * 10 + n);
                    if curstart.is_none() {
                        curstart = Some(c);
                    }
                }
                if !numeric || c == line.len() - 1 {
                    if let Some((start, num)) = curstart.zip(curnum) {
                        let end = if numeric { c } else { c - 1 };
                        let mut cnt_gear_pt = false;
                        for i in r.saturating_sub(1)..=r.saturating_add(1) {
                            for j in start.saturating_sub(1)..=end.saturating_add(1) {
                                if let Some(&adj_ch) = input.get(i).and_then(|l| l.get(j)) {
                                    if adj_ch == '*' && !cnt_gear_pt {
                                        let gear = gears
                                            .entry((i, j))
                                            .or_insert(Gear { ratio: 1, adj: 0 });
                                        gear.ratio *= num as usize;
                                        gear.adj += 1;
                                        cnt_gear_pt = true;
                                    }
                                }
                            }
                        }
                    }
                    curnum = None;
                    curstart = None;
                }
            }
        }
        ptsum = gears
            .into_values()
            .filter_map(|gear| (gear.adj == 2).then_some(gear.ratio))
            .sum::<usize>() as u32;
    }

    Ok(format!("{}", ptsum))
}

struct Gear {
    ratio: usize,
    adj: usize,
}
