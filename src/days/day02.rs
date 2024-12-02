use advent_of_code_2024::*;
use itertools::Itertools;

#[derive(PartialEq)]
enum Direction {
    Asc,
    Desc,
}

fn report_is_safe(report: &[u32]) -> bool {
    // Get direction by analyzing the first two numbers
    let direction = if report[0] < report[1] {
        Direction::Asc
    } else {
        Direction::Desc
    };

    report.iter().tuple_windows().all(|(a, b)| {
        let within_distance = a.abs_diff(*b) >= 1 && a.abs_diff(*b) <= 3;

        let current_direction = if a < b {
            Direction::Asc
        } else {
            Direction::Desc
        };

        within_distance && (current_direction == direction)
    })
}

fn generate_options(report: &[u32]) -> impl Iterator<Item = Vec<u32>> {
    // We don't need to include the original report in the options
    // because removing the first element of a safe report will always
    // reusult in a safe report
    let report = report.to_vec();
    (0..report.len()).map(move |i| {
        let mut option = report.clone();
        option.remove(i);
        option
    })
}

pub fn solve(context: &mut Context) {
    let input = context.input();
    // Parse each line into a vector of numbers
    let reports: Vec<Vec<u32>> = input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    let result = reports
        .iter()
        .filter(|report| report_is_safe(report))
        .count();

    context.set_sol1(result);

    let result = reports
        .iter()
        .filter(|report| generate_options(report).any(|option| report_is_safe(&option)))
        .count();

    context.set_sol2(result);
}
