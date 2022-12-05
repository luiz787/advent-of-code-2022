use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input");
    let (initial_state, procedure) = input.split_once("\n\n").unwrap();

    let number_of_stacks = get_number_of_stacks(initial_state);

    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); number_of_stacks];
    for (index, line) in initial_state.lines().enumerate() {
        if index == number_of_stacks {
            break;
        }
        for (stack_index, start_index) in (0..line.len()).step_by(4).enumerate() {
            let block = &line[start_index..start_index + 3];
            if block.starts_with("[") && block.ends_with("]") {
                stacks[stack_index].push_back(block.chars().nth(1).unwrap())
            }
        }
    }


    for step in procedure.lines() {
        let cleaned_step = step.replace("move", "").replace("from", "").replace("to", "");
        let cleaned_step: Vec<usize> = cleaned_step.split_ascii_whitespace().map(|i| i.parse().unwrap()).collect();
        let amount = cleaned_step[0];
        let from = cleaned_step[1] - 1;
        let to = cleaned_step[2] - 1;

        let mut to_move = VecDeque::new();
        for _ in 0..amount {            
            let curr = stacks[from].pop_front().unwrap();
            to_move.push_front(curr);
        }
        for char in to_move {
            stacks[to].push_front(char);
        }
    }

    stacks
        .iter()
        .map(|stack| stack.front().unwrap())
        .for_each(|c| print!("{}", c));

    println!();
}

fn get_number_of_stacks(initial_state: &str) -> usize {
    for line in initial_state.lines() {
        if !line.contains("["){
            return line.split_ascii_whitespace().last().unwrap().parse().unwrap();
        }
    }
    unreachable!();
}
