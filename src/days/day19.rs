use std::collections::HashMap;
use std::collections::HashSet;

use advent_of_code_2024::*;

pub fn solve(context: &mut Context) {
    let lines = context.input().to_owned();
    let towels = lines[0].split(", ").collect::<HashSet<_>>();
    let patterns = &lines[2..];

    let mut cache: HashMap<&str, usize> = HashMap::new();
    let mut feasibles = 0;
    let mut paths = 0;
    patterns.iter().for_each(|pattern| {
        let solutions = solve_towel(&towels, pattern, &mut cache);
        feasibles += if solutions > 0 { 1 } else { 0 };
        paths += solutions;
    });

    context.set_sol1(feasibles);
    context.set_sol2(paths);
}

fn solve_towel<'a>(
    towels: &HashSet<&str>,
    pattern: &'a str,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(&paths) = cache.get(pattern) {
        return paths;
    }

    let mut paths = 0;
    let mut i = 0;
    loop {
        if i == pattern.len() {
            break;
        }
        let prefix = &pattern[..=i];
        let suffix = &pattern[i + 1..];
        if towels.contains(prefix) {
            if suffix.is_empty() {
                paths += 1;
            } else {
                paths += solve_towel(towels, suffix, cache);
            }
        }
        i += 1;
    }

    cache.insert(pattern, paths);
    paths
}
