use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    for start in 0..input.len() - 14 {
        let window = &input[start..start+14];
        let mut set = HashSet::new();
        for char in window.chars() {
            set.insert(char);
        }
        
        if set.len() == 14 {
            println!("{}", start + 14);
            break;
        }
    }
}
