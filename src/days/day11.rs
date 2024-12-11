use advent_of_code_2024::*;

use std::collections::HashMap;

pub fn solve(context: &mut Context) {
    let stones_input: Vec<u64> = context.input()[0]
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut stones = stones_input.clone();
    for _ in 1..=25 {
        stones = apply_rules(stones);
    }
    context.set_sol1(stones.len());

    let mut cache: HashMap<(u64, usize), u64> = HashMap::new();
    let result = stones_input
        .into_iter()
        .map(|s| calculate_stones(s, 75, &mut cache))
        .sum::<u64>();
    context.set_sol2(result);
}

fn apply_rules(stones: Vec<u64>) -> Vec<u64> {
    // At worst the new list will be 2x the size of the original list
    let mut new_stones = Vec::with_capacity(stones.len() * 2);
    for stone in stones {
        if stone == 0 {
            new_stones.push(1);
            continue;
        }
        let number_of_digits = stone.ilog10() + 1;
        let is_even = number_of_digits % 2 == 0;
        if !is_even {
            new_stones.push(stone * 2024u64);
            continue;
        }

        let first_digits = stone / 10u64.pow(number_of_digits / 2);
        let last_digits = stone % 10u64.pow(number_of_digits / 2);
        new_stones.push(first_digits);
        new_stones.push(last_digits);
    }
    new_stones
}

fn calculate_stones(stone: u64, depth: usize, cache: &mut HashMap<(u64, usize), u64>) -> u64 {
    if depth == 0 {
        return 1;
    }

    if let Some(&result) = cache.get(&(stone, depth)) {
        return result;
    }

    let result = apply_rules(vec![stone])
        .iter()
        .map(|s| calculate_stones(*s, depth - 1, cache))
        .sum();
    cache.insert((stone, depth), result);
    result
}
