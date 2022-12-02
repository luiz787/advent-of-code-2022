use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    
    let result: u64 = input
        .trim()
        .split("\n")
        .map(|line| line.split_once(" ").unwrap())
        .map(|(opponent, me)| calculate_points(opponent, me))
        .sum();

    println!("{}", result);
}

fn calculate_points(opponent: &str, me: &str) -> u64 {
    let choice_points = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);

    let mut comparison_points = 0;
    if (opponent == "A" && me == "X") || (opponent == "B" && me == "Y") || (opponent == "C" && me == "Z")  {
        comparison_points = 3;
    } else if (opponent == "A" && me == "Y") || (opponent == "B" && me == "Z") || (opponent == "C" && me == "X") {
        comparison_points = 6;
    }

    
    choice_points[me] + comparison_points
}
