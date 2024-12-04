use advent_of_code_2024::*;

pub fn solve(context: &mut Context) {
    let map = Grid::parse(context.input());

    let mut number_of_xmas = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let point = Point::new(x as i64, y as i64);
            if map[point] == b'X' {
                // XMAS can only start with X
                for direction in DIAGONALS {
                    if traverse_direction(&map, point, direction) {
                        number_of_xmas += 1;
                    }
                }
            }
        }
    }

    context.set_sol1(number_of_xmas);

    let mut number_of_x_mas = 0;
    for y in 1..map.height-1 {
        for x in 1..map.width-1 {
            let point = Point::new(x as i64, y as i64);
            if map[point] == b'A' {
                // A needs to be in the middle
                let diag_1 = matches!((map[point + UPPER_LEFT], map[point + LOWER_RIGHT]), (b'M', b'S') | (b'S', b'M'));
                let diag_2 = matches!((map[point + UPPER_RIGHT], map[point + LOWER_LEFT]), (b'M', b'S') | (b'S', b'M'));

                if diag_1 && diag_2 {
                    number_of_x_mas += 1;
                }
            }
        }
    }

    context.set_sol2(number_of_x_mas);
}

fn traverse_direction(map: &Grid<u8>, start: Point, direction: Point) -> bool {
    const MAS: [u8; 3] = [b'M', b'A', b'S'];
    let mut current = start + direction;
    for letter in MAS {
        if !map.contains(current) || map[current] != letter {
            return false;
        }
        current += direction;
    }

    true
}
