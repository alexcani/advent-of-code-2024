use advent_of_code_2024::*;

use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve(context: &mut Context) {
    let map = Grid::parse(context.input());
    let mut visited: HashSet<Point> = HashSet::new();
    let mut groups: Vec<(u8, usize, usize, HashMap<Vertex, usize>)> = Vec::new(); // letter, area, perimeter, vertices
    map.iter().for_each(|(x, y)| {
        if visited.contains(&x) {
            return;
        }
        let (area, perimeter, vertices) = fill_group(&mut visited, &map, x, *y);
        groups.push((*y, area, perimeter, vertices));
    });
    let result = groups
        .iter()
        .map(|(_, area, perimeter, _)| area * perimeter)
        .sum::<usize>();
    context.set_sol1(result);

    let result = groups
        .iter()
        .map(|(letter, area, _, vertices)| {
            let corners = vertices
                .iter()
                .filter(|(_, &count)| count == 1 || count == 3)
                .count();
            let diagonals = vertices
                .iter()
                .filter(|(vertex, &count)| count == 2 && is_diagonal(vertex, &map, *letter))
                .count();
            let sides = corners + diagonals * 2;
            area * sides
        })
        .sum::<usize>();
    context.set_sol2(result);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vertex(Point, Point); // x, y; where each has an integer and a fractional value

fn fill_vertices(point: &Point, vertices: &mut HashMap<Vertex, usize>) {
    let v1 = Vertex(Point::new(point.x, 5), Point::new(point.y, 5)); // top right
    let v2 = Vertex(Point::new(point.x, 5), Point::new(point.y - 1, 5)); // bottom right
    let v3 = Vertex(Point::new(point.x - 1, 5), Point::new(point.y - 1, 5)); // bottom left
    let v4 = Vertex(Point::new(point.x - 1, 5), Point::new(point.y, 5)); // top left
    vertices
        .entry(v1)
        .and_modify(|count| *count += 1)
        .or_insert(1);
    vertices
        .entry(v2)
        .and_modify(|count| *count += 1)
        .or_insert(1);
    vertices
        .entry(v3)
        .and_modify(|count| *count += 1)
        .or_insert(1);
    vertices
        .entry(v4)
        .and_modify(|count| *count += 1)
        .or_insert(1);
}

fn fill_group(
    visited: &mut HashSet<Point>,
    map: &Grid<u8>,
    start: Point,
    letter: u8,
) -> (usize, usize, HashMap<Vertex, usize>) {
    let mut area = 0;
    let mut perimeter = 0;
    let mut vertices: HashMap<Vertex, usize> = HashMap::new();
    let mut to_visit: VecDeque<Point> = VecDeque::new();
    to_visit.push_back(start);
    while let Some(point) = to_visit.pop_front() {
        if visited.contains(&point) {
            continue;
        }

        visited.insert(point);
        fill_vertices(&point, &mut vertices);
        area += 1;

        ORTHOGONALS.iter().for_each(|dir| {
            let new_point = point + *dir;
            if !map.contains(new_point) {
                perimeter += 1;
            } else if map[new_point] == letter {
                to_visit.push_back(new_point);
            } else {
                perimeter += 1;
            }
        });
    }
    (area, perimeter, vertices)
}

fn is_diagonal(vertex: &Vertex, map: &Grid<u8>, letter: u8) -> bool {
    // For a vertex to be in a diagonal, it must belong to two points containing the letter,
    // and they have to be diagonal to each other
    // Build 4 points around the vertex
    let points = [  // example vertex (1.5, 0.5) -> results should be (2, 1), (2, 0), (1, 0), (1, 1)
        Point::new(vertex.0.x + 1, vertex.1.x + 1),  // 0 top right
        Point::new(vertex.0.x + 1, vertex.1.x),  // 1 bottom right
        Point::new(vertex.0.x, vertex.1.x),  // 2 bottom left
        Point::new(vertex.0.x, vertex.1.x + 1),  // 3 top left
    ];
    if points.iter().any(|p| !map.contains(*p)) {
        return false;
    }

    map[points[0]] == letter && map[points[2]] == letter || map[points[1]] == letter && map[points[3]] == letter
}
