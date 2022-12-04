fn main() {
    let input = include_str!("../input");
    let result = input.split("\n").into_iter()
        .filter(|line| line.len() > 0)
        .map(|line| line.split_once(",").unwrap())
        .map(|(r1, r2)| (to_range(r1), to_range(r2)))
        .filter(|((s1, e1), (s2, e2))| overlap(s1, e1, s2, e2))
        .count();

    println!("{}", result);
}

fn overlap(s1: &u32, e1: &u32, s2: &u32, e2: &u32) -> bool {
    (s1 <= s2 && e1 >= s2) || (s2 <= s1 && e2 >= s1)
}

fn to_range(s: &str) -> (u32, u32) {
    let (start, end) = s.split_once("-").unwrap();

    
    (start.parse().unwrap(), end.parse().unwrap())
}
