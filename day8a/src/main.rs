use std::collections::HashSet;

fn main() {
    let grid: Vec<Vec<_>> = include_str!("../input")
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - 48).collect())
        .collect();

    let mut vis: Vec<_> = grid
        .iter()
        .enumerate()
        .map(|(index, line)| visible_positions(index, &line))
        .collect();

    let transposed: Vec<_> = (0..grid[0].len())
        .map(|i| {
            grid.iter()
                .map(|inner| inner[i].clone())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut viscol: Vec<_> = transposed
        .iter()
        .enumerate()
        .map(|(index, line)| visible_positions_col(index, line))
        .collect();

    vis.append(&mut viscol);


    let mut resulting_set: HashSet<(usize, usize)> = HashSet::new();
    for item in vis {
        resulting_set.extend(item.iter());
    }


    println!("{}", resulting_set.len());
}

fn visible_positions(index: usize, line: &Vec<u8>) -> HashSet<(usize, usize)> {
    let mut visible_positions = HashSet::new();
    visible_positions.insert((index, 0));
    visible_positions.insert((index, line.len() - 1));
    for i in 1..line.len() {
        if line.iter().take(i).all(|v| v < &line[i]) {
            visible_positions.insert((index, i));
        }
    }

    for i in (1..line.len()).rev() {
        if line.iter().rev().take(line.len() - i - 1).all(|v| v < &line[i]) {
            visible_positions.insert((index, i));
        }
    }

    visible_positions
}

fn visible_positions_col(index: usize, line: &Vec<u8>) -> HashSet<(usize, usize)> {
    let mut visible_positions = HashSet::new();
    visible_positions.insert((0, index));
    visible_positions.insert((line.len() - 1, index));
    
    for i in 1..line.len() {
        if line.iter().take(i).all(|v| v < &line[i]) {
            visible_positions.insert((i, index));
        }
    }

    for i in (1..line.len()).rev() {
        if line.iter().rev().take(line.len() - i - 1).all(|v| v < &line[i]) {
            visible_positions.insert((i, index));
        }
    }

    visible_positions
}
