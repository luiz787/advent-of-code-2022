use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    let mut visited = HashSet::<(i32, i32)>::new();
    let initial_position: (i32, i32) = (0,0);
    let (mut hx, mut hy) = initial_position;
    let (mut tx, mut ty) = initial_position;
    for line in input.lines() {
        let (direction, amount) = line.split_once(" ").unwrap();
        let amount: i32 = amount.parse().unwrap();
        for _ in 0..amount {
            match direction {
                "R" => hx += 1,
                "L" => hx -= 1,
                "D" => hy += 1,
                "U" => hy -= 1,
                _ => continue
            }
            if hx.abs_diff(tx) > 1 {
                match direction {
                    "R" => tx += 1,
                    "L" => tx -= 1,
                    _ => panic!("problem")
                }
                ty = hy;
            } else if hy.abs_diff(ty) > 1{
                match direction {
                    "D" => ty += 1,
                    "U" => ty -= 1,
                    _ => panic!("problem")
                }
                tx = hx;
    
            }
            visited.insert((tx, ty)); 
        }
    }

    println!("{}", visited.len());
}
