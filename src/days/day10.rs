use advent_of_code_2024::*;
use itertools::Itertools;

pub fn solve(context: &mut Context) {
    let map = Grid::parse(context.input());
    let result: (usize, usize) = map
        .iter()
        .filter_map(|(pos, val)| match val {
            b'0' => Some(traverse(&map, pos)),
            _ => None,
        }).fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    context.set_sol1(result.0);
    context.set_sol2(result.1);
}

fn traverse(map: &Grid<u8>, start: Point) -> (usize, usize) {
    let mut nodes: Vec<(Point, u8)> = Vec::new(); // (coord, next step)
    nodes.push((start, b'1'));

    let mut trailheads = Vec::new();
    while let Some((pos, next)) = nodes.pop() {
        ORTHOGONALS.iter().for_each(|direction| {
            let next_pos = pos + *direction;
            if !map.contains(next_pos) {
                return;
            }

            if map[next_pos] == next {
                if next == b'9' {
                    trailheads.push(next_pos);
                } else {
                    nodes.push((next_pos, next + 1));
                }
            }
        });
    }

    (trailheads.iter().unique().count(), trailheads.len())
}
