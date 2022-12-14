use std::collections::HashSet;
use itertools::Itertools;

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
    let mut count = 0;
    loop {
        let (mut curr_x, mut curr_y) = (500, 0);
        while curr_y < max_y {
            if !rocks_positions.contains(&(curr_x, curr_y + 1)) {
                curr_y += 1;
            } else if !rocks_positions.contains(&(curr_x - 1, curr_y + 1)) {
                curr_x -= 1;
                curr_y += 1;
            } else if !rocks_positions.contains(&(curr_x + 1, curr_y + 1)){
                curr_x += 1;
                curr_y += 1;
            } else {
                count += 1;
                
                rocks_positions.insert((curr_x, curr_y));
                break;
            }
        }
        if curr_y >= max_y {
            break;
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
