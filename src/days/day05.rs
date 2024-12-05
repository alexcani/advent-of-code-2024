use advent_of_code_2024::*;

// The rule is: if the first and second elements are present in the sequence,
// then the first element must come at some point before the second element.
#[derive(Debug)]
struct DependencyPair(u8, u8);

pub fn solve(context: &mut Context) {
    let mut dependencies: Vec<DependencyPair> = Vec::new();
    let mut sequencies: Vec<Vec<u8>> = Vec::new();

    context.input().iter().for_each(|line| {
        if line.is_empty() {
            return;
        }
        if line.contains("|") {
            let mut parts = line.split("|");
            let first = parts.next().unwrap().parse::<u8>().unwrap();
            let second = parts.next().unwrap().parse::<u8>().unwrap();
            dependencies.push(DependencyPair(first, second));
        } else {
            sequencies.push(
                line.split(",")
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>(),
            );
        }
    });

    let result: u32 = sequencies
        .iter()
        .map(|sequence| {
            if is_valid_sequence(sequence, &dependencies) {
                sequence[sequence.len() / 2] as u32
            } else {
                0
            }
        })
        .sum();

    context.set_sol1(result);

    let result: u32 = sequencies
        .iter()
        .filter(|sequence| !is_valid_sequence(sequence, &dependencies))
        .map(|sequence| {
            let mut sequence = sequence.clone();
            fix_sequence_once(&mut sequence, &dependencies);
            while !is_valid_sequence(&sequence, &dependencies) {
                fix_sequence_once(&mut sequence, &dependencies);
            }
            sequence[sequence.len() / 2] as u32
        })
        .sum();

    context.set_sol2(result);
}

fn is_valid_sequence(sequence: &[u8], dependencies: &[DependencyPair]) -> bool {
    dependencies.iter().all(|DependencyPair(first, second)| {
        match (
            sequence.iter().position(|&x| x == *first),
            sequence.iter().position(|&x| x == *second),
        ) {
            (Some(first_index), Some(second_index)) => first_index < second_index,
            _ => true,
        }
    })
}

fn fix_sequence_once(sequence: &mut [u8], dependencies: &[DependencyPair]) {
    for DependencyPair(first, second) in dependencies {
        if let (Some(first_index), Some(second_index)) = (
            sequence.iter().position(|&x| x == *first),
            sequence.iter().position(|&x| x == *second),
        ) {
            if first_index >= second_index {
                sequence.swap(first_index, second_index);
                return;
            }
        };
    }
}
