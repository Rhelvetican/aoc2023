use anyhow::Result;

use crate::utils::get_input;

pub fn day_4(is_p2: bool) -> Result<String> {
    let input = get_input(4)?;
    let mut total_win = 0;
    if !is_p2 {
        for card in input.lines() {
            fn _calc_score(wins: u32) -> u32 {
                if wins == 0 {
                    0
                } else {
                    2_u32.pow(wins - 1)
                }
            }
            let card = Card::from_str(card);
            let total_num_of_winning_nums = card.total_wins();
            total_win += _calc_score(total_num_of_winning_nums);
        }
    } else {
        let cards = input.lines().map(Card::from_str).collect::<Vec<_>>();
        let mut win_amounts = vec![1u32; cards.len()];
        for card in &cards {
            let win_amount = card.total_wins();
            let multiplier = win_amounts[card.id - 1];
            for num in win_amounts[card.id..card.id + win_amount as usize].iter_mut() {
                *num += multiplier;
            }
        }
        total_win = win_amounts.iter().sum();
    }

    Ok(format!("{}", total_win))
}

#[derive(Default, Clone)]
struct Card {
    id: usize,
    winning_nums: Vec<u32>,
    your_nums: Vec<u32>,
}

impl Card {
    fn from_str(input: &str) -> Self {
        let parts = input.split(':').collect::<Vec<_>>();
        let id = parts[0]
            .replace("Card", "")
            .trim()
            .parse::<usize>()
            .unwrap_or(0);
        let parts = parts[1].split('|').collect::<Vec<_>>();
        let winning_nums = parts[0]
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap_or(0))
            .collect::<Vec<_>>();
        let your_nums = parts[1]
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap_or(0))
            .collect::<Vec<_>>();
        Self {
            id,
            winning_nums,
            your_nums,
        }
    }
    fn total_wins(&self) -> u32 {
        let mut total_num_of_winning_nums = 0;
        for your_num in &self.your_nums {
            if self.winning_nums.contains(your_num) {
                total_num_of_winning_nums += 1;
            }
        }
        total_num_of_winning_nums
    }
}
