use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    let result: u16 = input.split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| get_compartments(line))
        .map(|(first, second)| get_common_character_value(first, second))
        .sum();

    println!("{}", result);
}

fn get_compartments(line: &str) -> (&str, &str) {
    let first_compartment = &line[0..line.len() / 2];
    let second_compartment = &line[line.len()/2..line.len()];

    (first_compartment, second_compartment)
}

fn get_common_character_value(first_compartment: &str, second_compartment: &str) -> u16 {
    let first: HashSet<_> = first_compartment.chars()
    .collect();
    let second: HashSet<_> = second_compartment.chars().collect();

    let intersection = first.intersection(&second);

    let intersection: Vec<_> = intersection.collect();

    let value = intersection.into_iter().next().unwrap();
    if value.is_ascii_uppercase() {
        (*value as u16) - 38
    } else {
        (*value as u16) - 96
    }
}
