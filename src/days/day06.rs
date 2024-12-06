use advent_of_code_2024::*;

use std::collections::HashSet;

pub fn solve(context: &mut Context) {
  let mut map = Grid::parse(context.input());

  let mut visited_points = HashSet::new();

  let starting_pos = map.find(b'^').unwrap();
  visited_points.insert(starting_pos);

  let mut current_direction = UP;
  let mut current_pos = starting_pos;
  let mut next_pos = current_pos + current_direction;
  while map.contains(next_pos) {
    if map[next_pos] == b'#' {
      current_direction = current_direction.clockwise();
    } else {
      current_pos = next_pos;
      visited_points.insert(current_pos);
    }
    next_pos = current_pos + current_direction;
  }

  context.set_sol1(visited_points.len() as u32);

  visited_points.remove(&starting_pos);
  let n_possible_loops = visited_points.into_iter().filter(|&pos| {
    map[pos] = b'#';
    let loops = forms_loop(&map, &starting_pos);
    map[pos] = b'.';

    loops
  }).count();

  context.set_sol2(n_possible_loops);
}

#[derive(PartialEq, Eq, Hash)]
struct Vector(Point, Point); // point, direction

fn forms_loop(map: &Grid<u8>, starting_pos: &Point) -> bool {
  let mut visited_points = HashSet::new();

  let mut current_direction = UP;
  visited_points.insert(Vector(*starting_pos, current_direction));

  let mut current_pos = *starting_pos;
  let mut next_pos = current_pos + current_direction;
  while map.contains(next_pos) {
    if map[next_pos] == b'#' {
      current_direction = current_direction.clockwise();
    } else {
      current_pos = next_pos;
      let current_vector = Vector(current_pos, current_direction);
      if visited_points.contains(&current_vector) {
        return true;
      }
      visited_points.insert(current_vector);
    }
    next_pos = current_pos + current_direction;
  }

  false
}
