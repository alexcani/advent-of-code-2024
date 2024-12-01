use advent_of_code_2024::*;

pub fn solve(context: &mut Context) {
    let mut left_column: Vec<u32> = Vec::new();
    let mut right_column: Vec<u32> = Vec::new();

    context.input().iter().for_each(|line| {
        let split: Vec<&str> = line.split("   ").collect();

        left_column.push(split[0].parse().unwrap());
        right_column.push(split[1].parse().unwrap());
    });

    left_column.sort_unstable();
    right_column.sort_unstable();

    let result: u32 = std::iter::zip(&left_column, &right_column)
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    context.set_sol1(result);

    let mut result = 0;
    let mut r_index = 0;
    let len = right_column.len();
    let mut step_result = 0;
    let mut last_element = 0;
    'outer: for l in &left_column {
        if *l == last_element {  // acount for duplicates. funny enough the input has no duplicates, but the example does
            result += step_result;
            continue;
        }

        step_result = 0;
        last_element = *l;

        loop {
            let element = right_column[r_index];
            if element > *l {
                break;
            }
            if element == *l {
                step_result += l;
            }
            r_index += 1;
            if r_index == len {
                break 'outer;
            }
        }

        result += step_result;
    }

    context.set_sol2(result);
}
