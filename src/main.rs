mod days;
mod example;

use advent_of_code_2024::Context;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the day(s) to run as a command-line argument.");
    }

    let days: Vec<u8> = args[1..]
        .iter()
        .map(|x| {
            x.parse()
                .unwrap_or_else(|v| panic!("Not a valid day: {}", v))
        })
        .collect();

    let mut runtime = 0.0;

    for day in days {
        let func = get_day_solver(day);
        let input = get_day_input(day);

        let mut context = Context::new(input);
        func(&mut context);

        let mut elapsed_ms = 0.0;
        println!("=== Day {:02} ===", day);
        match context.sol1() {
            Some((sol, time)) => {
                println!("  · Part 1: {}", sol);
                println!("  · Elapsed: {:.4} ms", time);
                elapsed_ms += time;
            }
            None => println!("Part 1: Not implemented"),
        }
        match context.sol2() {
            Some((sol, time)) => {
                println!("  · Part 2: {}", sol);
                println!("  · Elapsed: {:.4} ms", time);
                elapsed_ms += time;
            }
            None => println!("Part 2: Not implemented"),
        }
        println!("Total: {:.4} ms\n", elapsed_ms);

        runtime += elapsed_ms;
    }

    println!("Total runtime: {:.4} ms", runtime);
}

fn get_day_solver(day: u8) -> fn(&mut Context) -> () {
    match day {
        1 => days::day01::solve,
        _ => unimplemented!(),
    }
}

fn get_day_input(day: u8) -> Vec<String> {
    // If environment variable EXAMPLE is set, use example input
    if env::var("EXAMPLE").is_ok() {
        return example::EXAMPLE
            .lines()
            .map(|l| l.trim().to_string())
            .collect::<Vec<String>>();
    }

    let path = format!("inputs/day{:02}.txt", day);
    match advent_of_code_2024::read_lines_as_vec(&path) {
        Ok(lines) => lines,
        Err(_) => {
            let path = format!("inputs/day{}.txt", day);
            match advent_of_code_2024::read_lines_as_vec(&path) {
                Ok(lines) => lines,
                Err(_) => panic!("Could not find input file"),
            }
        }
    }
}
