use advent_of_code_2024::*;

use regex::Regex;

struct Problem {
    x_equation: Point,
    y_equation: Point,
    prize: Point,
}

pub fn solve(context: &mut Context) {
    let button_regex = Regex::new(r"X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let prize_regex = Regex::new(r"X=([0-9]+), Y=([0-9]+)").unwrap();
    let problems: Vec<Problem> = context
        .input()
        .split(|x| x.is_empty())
        .map(|s| {
            let a = button_regex.captures(&s[0]).unwrap();
            let b = button_regex.captures(&s[1]).unwrap();
            let prize = prize_regex.captures(&s[2]).unwrap();

            Problem {
                x_equation: Point::new(a[1].parse().unwrap(), b[1].parse().unwrap()),
                y_equation: Point::new(a[2].parse().unwrap(), b[2].parse().unwrap()),
                prize: Point::new(prize[1].parse().unwrap(), prize[2].parse().unwrap()),
            }
        })
        .collect();

    let result: i64 = problems
        .iter()
        .map(solve_problem)
        .map(|solution| {
            if solution.0 < 0 || solution.1 < 0 || solution.0 > 100 || solution.1 > 100 {
                return 0;
            }
            solution.0 * 3 + solution.1
        })
        .sum();

    context.set_sol1(result);

    let result: i64 = problems
        .into_iter()
        .map(|mut p| {
            p.prize.x += 10000000000000;
            p.prize.y += 10000000000000;
            p
        })
        .map(|p| solve_problem(&p))
        .map(|solution| {
            if solution.0 < 0 || solution.1 < 0 {
                return 0;
            }
            solution.0 * 3 + solution.1
        })
        .sum();

    context.set_sol2(result);
}

fn solve_problem(problem: &Problem) -> (i64, i64) {
    let det =
        problem.x_equation.x * problem.y_equation.y - problem.x_equation.y * problem.y_equation.x;
    let x = (problem.prize.x * problem.y_equation.y - problem.prize.y * problem.x_equation.y) / det;
    let y =
        (-problem.prize.x * problem.y_equation.x + problem.prize.y * problem.x_equation.x) / det;

    if problem.x_equation.x * x + problem.x_equation.y * y != problem.prize.x
        || problem.y_equation.x * x + problem.y_equation.y * y != problem.prize.y
    {
        return (-1, -1);  // no solution in the integer domain
    }
    (x, y)
}
