use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    for start in 0..input.len() - 4 {
        let window = &input[start..start+4];
        let mut set = HashSet::new();
        for char in window.chars() {
            set.insert(char);
        }
        
        if set.len() == 4 {
            println!("{}", start + 4);
            break;
        }
    }
}
