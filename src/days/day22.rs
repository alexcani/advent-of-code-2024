use advent_of_code_2024::*;

use std::collections::{HashMap, HashSet};

pub fn solve(context: &mut Context) {
    let inputs = context
        .input()
        .iter()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut result = 0;
    for input in &inputs {
        let mut n = *input;
        for _ in 0..2000 {
            n = generate_new_number(n);
        }
        result += n;
    }
    context.set_sol1(result);

    // Generate all deltas for all inputs along with current price.
    // Meanwhile, for each new sequence update the cache with the new total for that sequence.
    let mut cache = HashMap::<(i8, i8, i8, i8), u32>::new();
    inputs.into_iter().for_each(|initial_number| {
        // Make sure we only count each sequence once for a given input
        let mut added = HashSet::new();

        // Make the first 4 deltas manually
        let n = initial_number;
        let p = (n % 10) as i8;
        let n1 = generate_new_number(n);
        let p1 = (n1 % 10) as i8;
        let n2 = generate_new_number(n1);
        let p2 = (n2 % 10) as i8;
        let n3 = generate_new_number(n2);
        let p3 = (n3 % 10) as i8;
        let n4 = generate_new_number(n3);
        let p4 = (n4 % 10) as i8;
        let mut deltas = (p1 - p, p2 - p1, p3 - p2, p4 - p3);
        *cache.entry(deltas).or_default() += p4 as u32;
        added.insert(deltas);

        // Generate the rest of the deltas
        let mut n = n4;
        for _ in 0..1996 {
            let old_n = n;
            n = generate_new_number(n);
            let old_p = (old_n % 10) as i8;
            let pn = (n % 10) as i8;
            deltas = (deltas.1, deltas.2, deltas.3, pn - old_p);
            if !added.contains(&deltas) {
                *cache.entry(deltas).or_default() += pn as u32;
                added.insert(deltas);
            }
        }
    });

    // Cache is now filled with all sequences and their totals.
    // Find the best total.
    let best_result = cache.values().max().unwrap();
    context.set_sol2(*best_result);
}

fn generate_new_number(n: u64) -> u64 {
    let mut n = ((n * 64) ^ n) % 16777216;
    n = ((n / 32) ^ n) % 16777216;
    n = ((n * 2048) ^ n) % 16777216;
    n
}
