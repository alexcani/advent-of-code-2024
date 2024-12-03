use advent_of_code_2024::*;

use regex::Regex;

pub fn solve(context: &mut Context) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let result: u32 = context
        .input()
        .iter()
        .map(|line| {
            re.captures_iter(line)
                .map(|c| c.extract())
                .map(|(_, [x, y])| x.parse::<u32>().unwrap() * y.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sum();

    context.set_sol1(result);

    let line = context.input().join("");
    let mut do_index = 0;
    let mut dont_index = line[do_index..].find("don't()").unwrap() + do_index;
    let mut result: u32 = 0;
    loop {
        // run mul() over the valid range do()...don't()
        result += re
            .captures_iter(&line[do_index..dont_index])
            .map(|c| c.extract())
            .map(|(_, [x, y])| x.parse::<u32>().unwrap() * y.parse::<u32>().unwrap())
            .sum::<u32>();

        // find the next do()...don't() range

        // Find the first do() after the last don't()
        match &line[dont_index..].find("do()") {
            Some(index) => {
                do_index = dont_index + index + 4;
            }
            None => break,
        }

        // Find the first don't() after the do()
        dont_index = match &line[do_index..].find("don't()") {
            Some(index) => do_index + index + 7,
            None => line.len(),
        };
    }

    context.set_sol2(result);
}
