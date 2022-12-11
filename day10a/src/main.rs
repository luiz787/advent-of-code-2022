use std::vec;

fn main() {
    let mut values = vec![1];
    let mut current_value = 1;
    for instruction in include_str!("../input").lines() {
        if instruction == "noop" {
            values.push(current_value);
        } else {
            let (_addx, amount) = instruction.split_once(" ").unwrap();
            let amount: i32 = amount.parse().unwrap();

            values.push(current_value);
            current_value += amount;
            values.push(current_value);
        }
    }

    let indices = (19..220).step_by(40);

    let result: i32 = indices.map(|idx| (idx as i32 + 1) * values[idx]).sum();

    println!("{}", result);
}
