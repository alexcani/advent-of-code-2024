use advent_of_code_2024::*;

use pathfinding::prelude::astar_bag;
use std::collections::HashMap;

pub fn solve(context: &mut Context) {
    let keymap: HashMap<char, usize> = [
        ('A', 0),
        ('1', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('0', 10),
        ('<', 1),
        ('>', 2),
        ('^', 3),
        ('v', 4),
    ]
    .into();

    let inputs = context
        .input()
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let (shortest_paths_directional, shortest_paths_numeric) = calculate_shortest_paths();

    // This is the root controller, the least cost to press is simply the length of the
    // shortest path found in the previous step.
    // Need to add 1 to the cost because the root controller has to press the button
    // and the shortest paths are just directions
    let mut first_keypad = Box::new(Keypad::new_directional());
    for (from, from_vec) in shortest_paths_directional.iter().enumerate() {
        for (to, paths) in from_vec.iter().enumerate() {
            first_keypad.least_costs[from][to] = paths[0].len() as u64 + 1;
        }
    }

    let mut second_keypad = Box::new(Keypad::new_directional());
    second_keypad.parent = Some(first_keypad);
    second_keypad.calculate_costs(&shortest_paths_directional);

    let mut numeric_keypad = Keypad::new_numeric();
    numeric_keypad.parent = Some(second_keypad);
    numeric_keypad.calculate_costs(&shortest_paths_numeric);

    let mut result = 0;
    let mut keypad = numeric_keypad;
    for input in &inputs {
        keypad.current_button = 0;  // already true by design of the problem, but just to be sure

        let mut cost = 0;
        for key in input {
            let button = keymap[key];
            cost += keypad.least_costs[keypad.current_button][button];
            keypad.current_button = button;
        }

        let numeric_code = match (
            input[0].to_digit(10),
            input[1].to_digit(10),
            input[2].to_digit(10),
        ) {
            (Some(a), Some(b), Some(c)) => a * 100 + b * 10 + c,
            _ => panic!("Invalid input {:?}", input),
        };
        result += numeric_code * cost as u32;
    }

    context.set_sol1(result);

    // Part 2, 24 extra directional keypads
    let mut keypad = Box::new(Keypad::new_directional());  // root controller
    for (from, from_vec) in shortest_paths_directional.iter().enumerate() {
        for (to, paths) in from_vec.iter().enumerate() {
            keypad.least_costs[from][to] = paths[0].len() as u64 + 1;
        }
    }
    for _ in 0..24 {
        let mut new_keypad = Box::new(Keypad::new_directional());
        new_keypad.parent = Some(keypad);
        new_keypad.calculate_costs(&shortest_paths_directional);
        keypad = new_keypad;
    };

    let mut numeric_keypad = Keypad::new_numeric();
    numeric_keypad.parent = Some(keypad);
    numeric_keypad.calculate_costs(&shortest_paths_numeric);

    let mut result = 0;
    let mut keypad = numeric_keypad;
    for input in inputs {
        keypad.current_button = 0;  // already true by design of the problem, but just to be sure

        let mut cost = 0;
        for key in &input {
            let button = keymap[key];
            cost += keypad.least_costs[keypad.current_button][button];
            keypad.current_button = button;
        }

        let numeric_code = match (
            input[0].to_digit(10),
            input[1].to_digit(10),
            input[2].to_digit(10),
        ) {
            (Some(a), Some(b), Some(c)) => a * 100 + b * 10 + c,
            _ => panic!("Invalid input {:?}", input),
        };
        result += numeric_code as u64 * cost;
    }

    context.set_sol2(result);
}

fn direction_to_button(direction: Point) -> usize {
    match direction {
        LEFT => 1,
        RIGHT => 2,
        UP => 3,
        DOWN => 4,
        _ => panic!("Invalid direction {:?}", direction),
    }
}

struct Keypad {
    // Least cost to PRESS button X when starting from button Y,
    // viewed from the perspective of the root controller
    least_costs: Vec<Vec<u64>>,
    parent: Option<Box<Keypad>>,
    current_button: usize,
    // buttons in this keypad, their neighbors and direction of the neighbors
    buttons: Vec<Vec<(usize, Point)>>,
    //buttons_map: HashMap<char, HashMap<char, Vec<char>>>,
}

impl Keypad {
    fn calculate_costs(&mut self, shortest_paths: &ShortestPaths) {
        // Based on the costs of the parent keypad, calculate the costs of this keypad
        // using the shortest paths for this keypad model
        if self.parent.is_none() {
            return;
        }
        let parent = self.parent.as_mut().unwrap();

        for (from, from_vec) in self.least_costs.iter_mut().enumerate() {
            for (to, cost) in from_vec.iter_mut().enumerate() {
                let mut least_cost = u64::MAX;
                for directions in &shortest_paths[from][to] {
                    let mut path_cost = 0;
                    for &direction in directions {
                        let button = direction_to_button(direction); // the button the parent controller has to press
                        path_cost += parent.least_costs[parent.current_button][button];  // cost of pressing the button
                        parent.current_button = button;
                    }

                    // Parent now has to press A to make us press the button we just moved to
                    path_cost += parent.least_costs[parent.current_button][0];
                    parent.current_button = 0;

                    // We have just pressed the button
                    if path_cost < least_cost {
                        least_cost = path_cost;
                    }
                }

                *cost = least_cost;
            }
        }
    }

    fn new_numeric() -> Self {
        let buttons = vec![
            vec![(10, LEFT), (3, UP)],                       // A
            vec![(4, UP), (2, RIGHT)],                       // 1
            vec![(1, LEFT), (5, UP), (3, RIGHT), (10, DOWN)],// 2
            vec![(2, LEFT), (6, UP), (0, DOWN)],             // 3
            vec![(1, DOWN), (7, UP), (5, RIGHT)],            // 4
            vec![(2, DOWN), (4, LEFT), (6, RIGHT), (8, UP)], // 5
            vec![(3, DOWN), (5, LEFT), (9, UP)],             // 6
            vec![(4, DOWN), (8, RIGHT)],                     // 7
            vec![(7, LEFT), (5, DOWN), (9, RIGHT)],          // 8
            vec![(8, LEFT), (6, DOWN)],                      // 9
            vec![(2, UP), (0, RIGHT)],                       // 0
        ];

        Keypad {
            least_costs: vec![vec![0; 11]; 11],
            parent: None,
            current_button: 0,
            buttons,
        }
    }

    fn new_directional() -> Self {
        let buttons = vec![
            vec![(3, LEFT), (2, DOWN)],           // A
            vec![(4, RIGHT)],                     // <
            vec![(0, UP), (4, LEFT)],             // >
            vec![(0, RIGHT), (4, DOWN)],          // ^
            vec![(1, LEFT), (2, RIGHT), (3, UP)], // v
        ];

        Keypad {
            least_costs: vec![vec![0; 5]; 5],
            parent: None,
            current_button: 0,
            buttons,
        }
    }

    fn direction(&self, from: usize, to: usize) -> Point {
        self.buttons[from]
            .iter()
            .find(|&&(n, _)| n == to)
            .map(|&(_, p)| p)
            .unwrap()
    }
}

type ShortestPaths = Vec<Vec<Vec<Vec<Point>>>>;
fn calculate_shortest_paths() -> (ShortestPaths, ShortestPaths) {
    // Since all instances of numeric or directional keypads are the same, we can use a generic one
    // to obtain all shortest paths from all buttons to all other buttons
    let keypad = Keypad::new_directional();
    let n_buttons = keypad.buttons.len();
    let mut shortest_paths_directional = vec![vec![vec![]; n_buttons]; n_buttons]; // [from][to] = [path1, path2, ...]
    for (from, from_vec) in shortest_paths_directional.iter_mut().enumerate() {
        for (to, to_vec) in from_vec.iter_mut().enumerate() {
            let (paths, _) = astar_bag(
                &from,
                |&b| {
                    keypad.buttons[b]
                        .iter()
                        .map(|&(n, _)| (n, 1))
                        .collect::<Vec<_>>()
                },
                |_| 0,
                |&b| b == to,
            )
            .unwrap();
            for path in paths {
                let mut directions = vec![];
                for (i, _) in path.iter().enumerate() {
                    if i > 0 {
                        let direction = keypad.direction(path[i - 1], path[i]);
                        directions.push(direction);
                    }
                }
                to_vec.push(directions);
            }
        }
    }

    let keypad = Keypad::new_numeric();
    let n_buttons = keypad.buttons.len();
    let mut shortest_paths_numeric = vec![vec![vec![]; n_buttons]; n_buttons]; // [from][to] = [path1, path2, ...]
    for (from, from_vec) in shortest_paths_numeric.iter_mut().enumerate() {
        for (to, to_vec) in from_vec.iter_mut().enumerate() {
            let (paths, _) = astar_bag(
                &from,
                |&b| {
                    keypad.buttons[b]
                        .iter()
                        .map(|&(n, _)| (n, 1))
                        .collect::<Vec<_>>()
                },
                |_| 0,
                |&b| b == to,
            )
            .unwrap();
            for path in paths {
                let mut directions = vec![];
                for (i, _) in path.iter().enumerate() {
                    if i > 0 {
                        let direction = keypad.direction(path[i - 1], path[i]);
                        directions.push(direction);
                    }
                }
                to_vec.push(directions);
            }
        }
    }
    (shortest_paths_directional, shortest_paths_numeric)
}
