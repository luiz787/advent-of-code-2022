use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{collections::HashMap, vec};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(
    adj_list: &HashMap<(usize, usize), Vec<(i32, i32)>>,
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<usize> {
    let mut dist: HashMap<(usize, usize), usize> = adj_list
        .clone()
        .into_keys()
        .map(|key| (key, usize::MAX))
        .collect();
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > dist[&position] {
            continue;
        }

        for (ex, ey) in &adj_list[&position] {
            let next = State {
                cost: cost + 1,
                position: (*ex as usize, *ey as usize),
            };

            if next.cost < dist[&next.position] {
                heap.push(next);
                if let Some(v) = dist.get_mut(&next.position) {
                    *v = next.cost;
                }
            }
        }
    }

    None
}

fn main() {
    let input = include_str!("../input").lines().collect::<Vec<_>>();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut values: Vec<Vec<u8>> = vec![vec![0; input[0].len()]; input.len()];
    let mut adjacency_map: HashMap<(usize, usize), Vec<(i32, i32)>> = HashMap::new();
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.char_indices() {
            if c == 'S' {
                start = (i, j);
                values[i][j] = 'a' as u8 - 97;
            } else if c == 'E' {
                end = (i, j);
                values[i][j] = 'z' as u8 - 97;
            } else {
                values[i][j] = c as u8 - 97;
            }
        }
    }
    for (i, line) in values.iter().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            let neighbors: Vec<_> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .map(|(dx, dy)| (i as i32 + dx, j as i32 + dy))
                .filter(|(nx, ny)| {
                    *nx >= 0 && *nx < input.len() as i32 && *ny >= 0 && *ny < line.len() as i32
                })
                .filter(|(nx, ny)| values[*nx as usize][*ny as usize] <= c + 1)
                .collect();

            adjacency_map.insert((i, j), neighbors);
        }
    }

    let result = shortest_path(&adjacency_map, start, end).unwrap();
    println!("{}", result);
}
