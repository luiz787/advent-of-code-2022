use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    let mut visited = HashSet::<(i32, i32)>::new();

    let mut positions = vec![(0, 0); 10];
    for line in input.lines() {
        let (direction, amount) = line.split_once(" ").unwrap();
        let amount: i32 = amount.parse().unwrap();

        for _ in 0..amount {
            match direction {
                "R" => positions[0].0 += 1,
                "L" => positions[0].0 -= 1,
                "D" => positions[0].1 += 1,
                "U" => positions[0].1 -= 1,
                _ => continue,
            }
            let mut moved_diagonally = false;

            for i in 1..positions.len() {
                let prev_x = positions[i - 1].0;
                let prev_y = positions[i - 1].1;

                let mut curr_x: i32 = positions[i].0;
                let mut curr_y: i32 = positions[i].1;
                
                if prev_x.abs_diff(curr_x) > 1 {
                    if prev_x > curr_x {
                        curr_x += 1;
                    } else {
                        curr_x -= 1;
                    }
                    if curr_y != prev_y && !moved_diagonally {
                        curr_y = prev_y;
                        moved_diagonally = true;
                    } else if curr_y != prev_y && moved_diagonally {
                        if prev_y > curr_y {
                            curr_y += 1;
                        } else {
                            curr_y -= 1;
                        }
                    } else {
                        moved_diagonally = false;
                    }
                }
                if prev_y.abs_diff(curr_y) > 1 {
                    if prev_y > curr_y {
                        curr_y += 1;
                    } else {
                        curr_y -= 1;
                    }
                    if curr_x != prev_x && !moved_diagonally {
                        curr_x = prev_x;
                        moved_diagonally = true;    
                    } else if curr_x != prev_x && moved_diagonally {
                        if prev_x > curr_x {
                            curr_x += 1;
                        } else {
                            curr_x -= 1;
                        }
                    } else {
                        moved_diagonally = false;
                    }
                }
                positions[i] = (curr_x, curr_y);
                if i == 9 {
                    visited.insert((curr_x, curr_y));
                }
            }
        }
    }

    println!("{}", visited.len());
}
