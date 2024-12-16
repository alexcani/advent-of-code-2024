use advent_of_code_2024::*;
use pathfinding::prelude::yen;
use std::collections::HashSet;

pub fn solve(context: &mut Context) {
    let map = Grid::parse(context.input());
    let start = map.find(b'S').unwrap();
    let end = map.find(b'E').unwrap();

    let result = yen(
        &(start, RIGHT),
        |pos| {
            let mut moves = vec![
                ((pos.0, pos.1.clockwise()), 1000),
                ((pos.0, pos.1.counter_clockwise()), 1000),
            ];
            let next = pos.0 + pos.1;
            if map[next] == b'.' || map[next] == b'E' {
                moves.push(((next, pos.1), 1));
            }
            moves
        },
        |pos| pos.0 == end,
        4, // there are 4 paths in the input
    );

    let cost = result[0].1;
    context.set_sol1(cost);

    let mut points = HashSet::new();
    for result in &result {
        if result.1 != cost {
            // only consider the shortest paths, to accomodate example and input in the same code
            break;
        }
        for node in &result.0 {
            points.insert(node.0);
        }
    }
    context.set_sol2(points.len());
}
