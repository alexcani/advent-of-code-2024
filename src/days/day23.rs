use advent_of_code_2024::*;

use std::collections::{HashMap, HashSet};

pub fn solve(context: &mut Context) {
    let mut nodes: HashMap<String, HashSet<String>> = HashMap::new();
    context.input().iter().for_each(|line| {
        let mut pcs = line.split('-');
        let pc1 = pcs.next().unwrap().to_owned();
        let pc2 = pcs.next().unwrap().to_owned();
        nodes.entry(pc1.clone()).or_default().insert(pc2.clone());
        nodes.entry(pc2).or_default().insert(pc1);
    });

    // Find groups of 3 interconnected nodes
    let mut groups = HashSet::new();
    for (node, neighbours) in &nodes {
        for neighbour in neighbours {
            let neighbours_of_neighbour = nodes.get(neighbour).unwrap();
            for common_neighbour in neighbours.intersection(neighbours_of_neighbour) {
                let mut group = vec![node, neighbour, common_neighbour];
                group.sort();  // ensures uniqueness
                groups.insert(group);
            }
        }
    }

    let groups_with_t = groups.iter().filter(|group| group.iter().any(|node| node.starts_with('t'))).count();
    context.set_sol1(groups_with_t);

    // Find largest fully interconnected group
    let mut groups = HashSet::new();
    for node in nodes.keys() {
        let mut group = HashSet::from([node.clone()]);
        let mut stack = vec![node];
        while let Some(current) = stack.pop() {
            for neighbour in nodes.get(current).unwrap() {
                let neighbours_of_neighbour = nodes.get(neighbour).unwrap();
                if group.is_subset(neighbours_of_neighbour) {
                    group.insert(neighbour.clone());
                    stack.push(neighbour);
                }

            }
        }
        let mut group: Vec<_> = group.into_iter().collect();
        group.sort();  // ensures uniqueness
        groups.insert(group);
    }

    let largest_group = groups.iter().max_by_key(|group| group.len()).unwrap();
    context.set_sol2(largest_group.join(","));
}
