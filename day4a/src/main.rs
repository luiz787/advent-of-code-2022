fn main() {
    let input = include_str!("../input");
    let result = input.split("\n").into_iter()
        .filter(|line| line.len() > 0)
        .map(|line| line.split_once(",").unwrap())
        .map(|(r1, r2)| (to_range(r1), to_range(r2)))
        .filter(|((s1, e1), (s2, e2))| (s1 <= s2 && e1 >= e2) || (s2 <= s1 && e2 >= e1))
        .count();

    println!("{}", result);
}

fn to_range(s: &str) -> (u32, u32) {
    let (start, end) = s.split_once("-").unwrap();

    
    (start.parse().unwrap(), end.parse().unwrap())
}
