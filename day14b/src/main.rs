use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let mut rocks_positions: HashSet<(i32, i32)> = include_str!("../input")
        .lines()
        .flat_map(|line| {
            line.split(" -> ").tuple_windows().flat_map(|(a, b)| {
                let (sx, sy) = extract_coords(a);
                let (ex, ey) = extract_coords(b);

                if sx == ex {
                    (sy.min(ey)..=sy.max(ey))
                        .map(|y| (sx, y))
                        .collect::<Vec<_>>()
                } else {
                    (sx.min(ex)..=sx.max(ex))
                        .map(|x| (x, sy))
                        .collect::<Vec<_>>()
                }
            })
        })
        .collect();

    let max_y = rocks_positions.iter().map(|(_, y)| *y).max().unwrap();
    let floor_y = max_y + 2;
    let mut count = 0;
    'outer: loop {
        let (mut curr_x, mut curr_y) = (500, 0);
        let mut iters = 0;
        while curr_y < floor_y {
            if curr_y == floor_y - 1 {
                count += 1;
                rocks_positions.insert((curr_x, curr_y));
                break;
            } else if !rocks_positions.contains(&(curr_x, curr_y + 1)) {
                curr_y += 1;
            } else if !rocks_positions.contains(&(curr_x - 1, curr_y + 1)) {
                curr_x -= 1;
                curr_y += 1;
            } else if !rocks_positions.contains(&(curr_x + 1, curr_y + 1)) {
                curr_x += 1;
                curr_y += 1;
            } else {
                count += 1;
                if iters == 0 {
                    break 'outer;
                }
                rocks_positions.insert((curr_x, curr_y));
                break;
            }
            iters += 1;
        }
    }

    println!("{}", count);
}

fn extract_coords(position: &str) -> (i32, i32) {
    position
        .split_once(",")
        .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
        .unwrap()
}
