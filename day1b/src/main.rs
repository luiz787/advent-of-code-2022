use std::cmp::Reverse;

fn main() {
    let input = include_str!("../input");
    let mut calories: Vec<u64> = input
        .trim()
        .split("\n\n")
        .map(|s| s.split("\n")
            .map(|ss| ss.trim().parse::<u64>().unwrap())
            .sum()
        )
        .collect();

    calories.sort_by_key(|elem| Reverse(*elem));

    let top_3_calories: u64 = calories.iter()
        .take(3)
        .sum();
        
    println!("{}", top_3_calories);
}
