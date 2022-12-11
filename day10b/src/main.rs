use std::vec;

fn main() {
    let mut values = vec![1];
    let mut current_value: i32 = 1;
    let mut cycle = 0;
    let mut output = String::new();
    for instruction in include_str!("../input").lines() {
        if current_value.abs_diff(cycle) < 2 {
            output += "#";
        } else {
            output += ".";
        }
        if instruction == "noop" {
            values.push(current_value);
            cycle += 1;
            cycle = cycle % 40;
        } else {
            let (_addx, amount) = instruction.split_once(" ").unwrap();
            let amount: i32 = amount.parse().unwrap();
            cycle += 1;
            cycle = cycle % 40;
            values.push(current_value);

            if current_value.abs_diff(cycle) < 2 {
                output += "#";
            } else {
                output += ".";
            }

            current_value += amount;
            cycle += 1;
            cycle = cycle % 40;

            values.push(current_value);
        }
    }

    for i in (0..output.len()).step_by(40) {
        println!("{}", &output[i..i+40]);
    }
}
