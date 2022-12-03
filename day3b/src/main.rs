use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input");
    let result: u16 = input.split("\n")
        .filter(|line| line.len() > 0)
        .chunks(3)
        .into_iter()
        .map(|group| group
            .map(|line| line.chars().collect::<HashSet<_>>())
            .reduce(|acc, item| acc.intersection(&item).cloned().collect())
            .map(|set| set.into_iter().collect::<Vec<_>>().into_iter().next().unwrap())
            .map(|char| if char.is_ascii_uppercase() {
                (char as u16) - 38
            } else {
                (char as u16) - 96
            })
            .unwrap()
        )
        
        .sum();

    println!("{}", result);
}
