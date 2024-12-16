use advent_of_code_2024::*;
use pathfinding::prelude::astar_bag;
use std::collections::HashSet;

pub fn solve(context: &mut Context) {
    let map = Grid::parse(context.input());
    let start = map.find(b'S').unwrap();
    let end = map.find(b'E').unwrap();

    let result = astar_bag(
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
        |x| {x.0.manhattan_distance(&end)},
        |pos| pos.0 == end
    ).unwrap();

    let cost = result.1;
    context.set_sol1(cost);

    let paths = result.0;
    let points = paths.flatten().map(|node| {
        node.0
    }).collect::<HashSet<_>>();
    context.set_sol2(points.len());
}
