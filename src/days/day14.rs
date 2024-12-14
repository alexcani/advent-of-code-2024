use advent_of_code_2024::*;

use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Robot {
    initial_pos: Point,
    velocity: Point,
}

pub fn solve(context: &mut Context) {
    let re = Regex::new(r"p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
    let mut robots = context.input().iter().map(|line|{
        let caps = re.captures(line).unwrap();
        Robot {
            initial_pos: Point::new(caps[1].parse().unwrap(), caps[2].parse().unwrap()),
            velocity: Point::new(caps[3].parse().unwrap(), caps[4].parse().unwrap()),
        }
    }).collect::<Vec<_>>();

    let (map_width, map_height) = if context.is_example() {
        (11, 7)
    } else {
        (101, 103)
    };

    let result = solve_part_1(&robots, 100, map_width, map_height);
    context.set_sol1(result);

    let result = solve_part_2(&mut robots, map_width, map_height);
    context.set_sol2(result);
}

fn solve_part_1(robots: &[Robot], duration: i64, map_width: i64, map_height: i64) -> u32 {
    let (mid_width, mid_height) = (map_width / 2, map_height / 2);
    let mut robots_in_quadrant = [0u32; 4];
    for robot in robots {
        let final_x = (((robot.initial_pos.x + robot.velocity.x * duration) % map_width) + map_width) % map_width;
        let final_y = (((robot.initial_pos.y + robot.velocity.y * duration) % map_height) + map_height) % map_height;
        if final_x == mid_width || final_y == mid_height {
            continue;
        }
        let upper = final_y < mid_height;
        let left = final_x < mid_width;
        let quadrant = match (upper, left) {
            (true, true) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (false, false) => 3,
        };
        robots_in_quadrant[quadrant] += 1;
    }
    robots_in_quadrant.iter().product()
}

fn solve_part_2(robots: &mut [Robot], map_width: i64, map_height: i64) -> u64 {
    let mut map = vec![0u32; (map_width * map_height) as usize];
    let duration = 7000;

    let mut min_entropy = f64::MAX;
    let mut min_entropy_time = 0;
    for instant in 0..duration {
        for robot in robots.iter_mut() {
            // Move the robot and updated the map
            robot.initial_pos.x = (((robot.initial_pos.x + robot.velocity.x) % map_width) + map_width) % map_width;
            robot.initial_pos.y = (((robot.initial_pos.y + robot.velocity.y) % map_height) + map_height) % map_height;
            let index = (robot.initial_pos.y * map_width + robot.initial_pos.x) as usize;
            map[index] += 1;
        }

        // Calculate the entropy
        let entropy = calculate_entropy(&map);
        if entropy < min_entropy {
            min_entropy = entropy;
            min_entropy_time = instant + 1;
        }

        map.iter_mut().for_each(|x| *x = 0);
    }

    min_entropy_time
}

fn calculate_entropy(map: &[u32]) -> f64 {
    let total = map.len() as f64;
    // sparsity index
    map.iter().filter(|&&count| count == 0).count() as f64 / total
}
