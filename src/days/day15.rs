use advent_of_code_2024::*;

use std::collections::VecDeque;
use itertools::Itertools;

pub fn solve(context: &mut Context) {
    let mut lines = context.input().split(|line| line.is_empty());
    let orig_map = Grid::parse(lines.next().unwrap());
    let move_sequence: Vec<Point> = lines
        .next()
        .unwrap()
        .iter()
        .flat_map(|line| line.bytes().map(Point::from))
        .collect();

    let mut map = orig_map.clone();
    let mut pos = orig_map.find(b'@').unwrap();
    for dir in &move_sequence {
        let new_pos = pos + *dir;
        match map[new_pos] {
            b'.' => {
                map[pos] = b'.';
                map[new_pos] = b'@';
                pos = new_pos;
            }
            b'#' => (),
            b'O' => {
                if let Some(free) = find_next_free(&map, new_pos, *dir) {
                    map[pos] = b'.';
                    map[new_pos] = b'@';
                    map[free] = b'O';
                    pos = new_pos;
                }
            }
            _ => unreachable!(),
        }
    }

    let result: i64 = map
        .iter()
        .filter_map(|(pos, &y)| {
            if y == b'O' {
                Some(pos.x + 100 * pos.y)
            } else {
                None
            }
        })
        .sum();
    context.set_sol1(result);

    let mut map = build_new_map(orig_map);
    let mut pos = map.find(b'@').unwrap();
    for dir in &move_sequence {
        let new_pos = pos + *dir;
        match map[new_pos] {
            b'.' => {
                map[pos] = b'.';
                map[new_pos] = b'@';
                pos = new_pos;
            }
            b'#' => (),
            b'[' | b']' => handle_box_colision(&mut map, &mut pos, *dir),
            _ => unreachable!(),
        }
    }

    let result: i64 = map
        .iter()
        .filter_map(|(pos, &y)| {
            if y == b'[' {
                Some(pos.x + 100 * pos.y)
            } else {
                None
            }
        })
        .sum();
    context.set_sol2(result);
}

fn find_next_free(map: &Grid<u8>, pos: Point, dir: Point) -> Option<Point> {
    let mut pos = pos + dir;
    loop {
        match map[pos] {
            b'.' => return Some(pos),
            b'#' => return None,
            _ => (),
        }
        pos += dir;
    }
}

fn build_new_map(map: Grid<u8>) -> Grid<u8> {
    let mut new_map = Grid::<u8> {
        width: map.width * 2,
        height: map.height,
        data: vec![0; map.width * 2 * map.height],
    };
    let mut new_map_iter = new_map.data.iter_mut();
    for (_, &value) in map.into_iter() {
        match value {
            b'.' => {
                *new_map_iter.next().unwrap() = b'.';
                *new_map_iter.next().unwrap() = b'.';
            }
            b'#' => {
                *new_map_iter.next().unwrap() = b'#';
                *new_map_iter.next().unwrap() = b'#';
            }
            b'O' => {
                *new_map_iter.next().unwrap() = b'[';
                *new_map_iter.next().unwrap() = b']';
            }
            b'@' => {
                *new_map_iter.next().unwrap() = b'@';
                *new_map_iter.next().unwrap() = b'.';
            }
            _ => unreachable!(),
        }
    }
    new_map
}

fn handle_box_colision(map: &mut Grid<u8>, pos: &mut Point, dir: Point) {
    if dir == RIGHT || dir == LEFT {
        // horizontal works the same as before, just with boxes of length 2
        if let Some(free) = find_next_free(map, *pos, dir) {
            let distance = free.manhattan_distance(pos);
            for i in (0..distance).rev() {
                let temp = map[*pos + dir * i];
                map[*pos + dir * i] = map[*pos + dir * (i + 1)];
                map[*pos + dir * (i + 1)] = temp;
            }
            *pos += dir;
        }
        return;
    }

    // vertical
    let next = *pos + dir;
    let mut to_visit = VecDeque::new();
    to_visit.push_back(next);
    if map[next] == b'[' {
        to_visit.push_back(next + RIGHT);
    } else {
        to_visit.push_back(next + LEFT);
    };

    let mut elements = Vec::new();
    while let Some(point) = to_visit.pop_front() {
        let next = point + dir;
        match (map[point], map[next]) {
            (b'[', b'[') | (b']', b']') => {
                // boxes are aligned, push only next position, since the other will be pushed by the next iteration
                to_visit.push_back(next);
                elements.push(point);
            }
            (b'[', b']') => {
                // boxes are not aligned, push both positions of next box
                to_visit.push_back(next);
                to_visit.push_back(next + LEFT);
                elements.push(point);
            }
            (b']', b'[') => {
                // boxes are not aligned, push both positions of next box
                to_visit.push_back(next);
                to_visit.push_back(next + RIGHT);
                elements.push(point);
            }
            (_, b'#') => return,
            (_, b'.') => elements.push(point),
            (b'@', _) => unreachable!(
                "Invalid state: {} {}\n{}",
                map[point] as char, map[next] as char, map
            ),
            _ => to_visit.push_back(next),
        }
    }

    // If we reach this point, we can move the box chain one step in the direction
    elements.iter().rev().unique().for_each(|&point| {
        let next = point + dir;
        let temp = map[point];
        map[point] = map[next];
        map[next] = temp;
    });

    // Move the robot
    map[*pos] = b'.';
    map[*pos + dir] = b'@';

    *pos += dir;
}
