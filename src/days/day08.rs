use advent_of_code_2024::*;

use std::collections::HashSet;
use std::collections::HashMap;

use itertools::Itertools;
use gcd::Gcd;

pub fn solve(context: &mut Context) {
  let map = Grid::parse(context.input());

  let mut antennas: HashMap<u8, Vec<Point>> = HashMap::new();
  for (point, value) in &map {  // gather antennas by type
    if *value == b'.'
    {
      continue;
    }
    antennas.entry(*value).or_default().push(point);
  }

  let mut antinodes = HashSet::new();

  for antenna_points in antennas.values() {
    antenna_points.iter().combinations(2).for_each(|pair| {
      let p1 = pair[0];
      let p2 = pair[1];
      let distance = *p2 - *p1;
      let antinode1 = *p2 + distance;
      let antinode2 = *p1 - distance;
      if map.contains(antinode1) {
        antinodes.insert(antinode1);
      }
      if map.contains(antinode2) {
        antinodes.insert(antinode2);
      }
    });
  }

  context.set_sol1(antinodes.len());

  antinodes.clear();
  for antenna_points in antennas.values() {
    antenna_points.iter().combinations(2).for_each(|pair| {
      let p1 = pair[0];
      let p2 = pair[1];
      let mut distance = *p2 - *p1;

      let gcd = distance.x.unsigned_abs().gcd(distance.y.unsigned_abs());
      distance.x /= gcd as i64;
      distance.y /= gcd as i64;

      let mut antinode = *p2;
      while map.contains(antinode) {
        antinodes.insert(antinode);
        antinode += distance;
      }

      let mut antinode = *p2 - distance;  // start from p2 to count space between p1 and p2
      while map.contains(antinode) {
        antinodes.insert(antinode);
        antinode -= distance;
      }
    });
  }

  context.set_sol2(antinodes.len());
}
