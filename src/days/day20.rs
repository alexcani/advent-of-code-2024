use std::collections::HashMap;

use advent_of_code_2024::*;

pub fn solve(context: &mut Context) {
    let map = Grid::parse(context.input());
    let start = map.find(b'S').unwrap();
    let minimum_save = if context.is_example() { 50 } else { 100 };

    // Since there's only 1 continuous sequence of . with no dead ends from start to finish,
    // the distance from the start to the end is simply the number of . tiles
    // + 1 for the E tile
    let base_time = 1 + map.data.iter().filter(|&&c| c == b'.').count() as i32;

    let mut map_distances = HashMap::new();

    // Assign distance until end to all points
    let mut next = vec![start];
    let mut current_distance = base_time;
    while let Some(point) = next.pop() {
        map_distances.insert(point, current_distance);
        current_distance -= 1;
        for &dir in ORTHOGONALS.iter() {
            let next_point = point + dir;
            if !map_distances.contains_key(&next_point) && map[next_point] != b'#' {
                next.push(next_point);
            }
        }
    }

    let mut result = 0;

    map.iter()
        .filter(|&c| c.1 == &b'.' || c.1 == &b'S')
        .for_each(|pos| {
            // Get the neighbours that are walls
            ORTHOGONALS
                .iter()
                .filter(|&dir| map[pos.0 + *dir] == b'#')
                .for_each(|&neighbour| {
                    let step_2 = pos.0 + neighbour + neighbour;
                    if !map.contains(step_2) || map[step_2] == b'#' {
                        return;
                    }

                    let time_start = map_distances[&pos.0];
                    let time_end = map_distances[&step_2];

                    let time_saved = time_start - time_end - 2;
                    if time_saved >= minimum_save {
                        result += 1;
                    }
                });
        });

    context.set_sol1(result);

    // Part 2, cheats can be up to 20 steps long
    let mut result = 0;
    map.iter()
        .filter(|&c| c.1 == &b'.' || c.1 == &b'S')
        .for_each(|pos| {
            let entry = &pos.0;

            let possible_exits = (-20..=20)
                .flat_map(|x| (-20..=20).map(move |y| Point::new(x, y) + *entry))
                .filter(|p| {
                    map.contains(p)
                        && map[p] != b'#'
                        && p != entry
                        && entry.manhattan_distance(p) <= 20
                })
                .collect::<Vec<_>>();

            possible_exits.iter().for_each(|exit| {
                let time_start = map_distances[entry];
                let time_end = map_distances[exit];
                let cheat_length = entry.manhattan_distance(exit) as i32;
                let time_saved = time_start - time_end - cheat_length;
                if time_saved >= minimum_save {
                    result += 1;
                }
            });
        });
    context.set_sol2(result);
}
