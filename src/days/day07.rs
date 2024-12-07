use advent_of_code_2024::*;

struct Equation {
    result: u64,
    operands: Vec<u64>,
}

pub fn solve(context: &mut Context) {
    let equations: Vec<Equation> = context
        .input()
        .iter()
        .map(|line| {
            let mut parts = line.split(":");
            let result = parts.next().unwrap().parse::<u64>().unwrap();
            let operands = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
            Equation { result, operands }
        })
        .collect();

    let result: u64 = equations.iter().filter_map(solvable_2_operands).sum();
    context.set_sol1(result);

    let result: u64 = equations.iter().filter_map(solvable_3_operands).sum();
    context.set_sol2(result);
}

fn solvable_2_operands(equation: &Equation) -> Option<u64> {
    if solve_2_operands(
        equation.result,
        equation.operands[0],
        &equation.operands[1..],
    ) {
        Some(equation.result)
    } else {
        None
    }
}

fn solve_2_operands(target: u64, current: u64, operands: &[u64]) -> bool {
    let sum = current + operands[0];
    let mult = current * operands[0];

    // end of the recursion
    if operands.len() == 1 {
        return sum == target || mult == target;
    }

    let mult_possible = if mult <= target {
        solve_2_operands(target, mult, &operands[1..])
    } else {
        false
    };

    let sum_possible = if sum <= target {
        solve_2_operands(target, sum, &operands[1..])
    } else {
        false
    };

    sum_possible || mult_possible
}

fn solvable_3_operands(equation: &Equation) -> Option<u64> {
    if solve_3_operands(
        equation.result,
        equation.operands[0],
        &equation.operands[1..],
    ) {
        Some(equation.result)
    } else {
        None
    }
}

fn concat(a: u64, b: u64) -> u64 {
  let b_digits = (b as f64).log10().floor() as u32 + 1;
  a * 10u64.pow(b_digits) + b
}

fn solve_3_operands(target: u64, current: u64, operands: &[u64]) -> bool {
    let sum = current + operands[0];
    let mult = current * operands[0];
    let concat = concat(current, operands[0]);

    // end of the recursion
    if operands.len() == 1 {
        return sum == target || mult == target || concat == target;
    }

    let mult_possible = if mult <= target {
        solve_3_operands(target, mult, &operands[1..])
    } else {
        false
    };

    let sum_possible = if sum <= target {
        solve_3_operands(target, sum, &operands[1..])
    } else {
        false
    };

    let concat_possible = if concat <= target {
        solve_3_operands(target, concat, &operands[1..])
    } else {
        false
    };

    sum_possible || mult_possible || concat_possible
}
