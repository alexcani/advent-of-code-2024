use advent_of_code_2024::*;

pub fn solve(context: &mut Context) {
    let mut keys: Vec<[i8; 5]> = Vec::new();
    let mut locks: Vec<[i8; 5]> = Vec::new();
    context.input().split(|line| line.is_empty()).for_each(|group| {
        let mut value = [-1; 5];  // start on -1 to counter the full line that defines key vs lock
        for line in group {
            let bytes = line.as_bytes();
            for i in 0..5 {
                if bytes[i] == b'#' {
                    value[i] += 1;
                }
            }
        }
        if group[0] == *"#####" {
            locks.push(value);
        } else {
            keys.push(value);
        }
    });

    let mut matches = 0;
    for lock in &locks {
        for key in &keys {
            if (0..5).all(|i| key[i] + lock[i] <= 5) {
                matches += 1;
            }
        }
    }
    context.set_sol1(matches);
}
