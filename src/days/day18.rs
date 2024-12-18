use advent_of_code_2024::*;

use pathfinding::prelude::dijkstra;

pub fn solve(context: &mut Context) {
    let map_orig = if context.is_example() {
        Grid::new(7, 7, b'.')
    } else {
        Grid::new(71, 71, b'.')
    };
    let points = context.input().iter().map(|line| {
        let mut split = line.split(',');
        Point::new(split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap())
    }).collect::<Vec<_>>();

    let n = if context.is_example() {
        12
    } else {
        1024
    };

    let end = if context.is_example() {
        Point::new(6, 6)
    } else {
        Point::new(70, 70)
    };

    let map = prepare_map(map_orig.clone(), &points, n);
    let cost = find_path(&map, ORIGIN, end).unwrap();
    context.set_sol1(cost);

    // Part 2, do bisecting search to find the first n that blocks the path
    let mut low = 0;
    let mut high = points.len();
    while low < high {
        let mid = (low + high) / 2;
        let map = prepare_map(map_orig.clone(), &points, mid);
        if find_path(&map, ORIGIN, end).is_some() {
            low = mid + 1;
        } else {
            high = mid
        }
    }
    let blocking_point = points[low-1];
    context.set_sol2(format!("{},{}", blocking_point.x, blocking_point.y));
}

fn find_path(map: &Grid<u8>, start: Point, end: Point) -> Option<usize> {
    dijkstra(&start, |&p| {
        ORTHOGONALS.iter().filter_map(|&n| {
            let np = p + n;
            if map.contains(np) && map[np] != b'#' {
                Some((np, 1))
            } else {
                None
            }
        }).collect::<Vec<_>>()
    }, |&p| p == end).map(|(_, cost)| cost)
}

fn prepare_map(mut map: Grid<u8>, points: &[Point], n: usize) -> Grid<u8> {
    points.iter().take(n).for_each(|p| {
        map[p] = b'#';
    });
    map
}
