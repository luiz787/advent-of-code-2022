fn main() {
    let input = include_str!("../input");
    let max_calories: u64 = input
        .trim()
        .split("\n\n")
        .map(|s| s.split("\n")
            .map(|ss| ss.trim().parse::<u64>().unwrap())
            .sum()
        ).max()
        .unwrap();
    println!("{}", max_calories);
}
