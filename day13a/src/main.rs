use std::cmp::Ordering;

#[derive(Debug)]
enum Packet {
    List(Vec<Packet>),
    Integer(u8),
}

impl Packet {
    fn new(value: &str) -> Packet {
        if value.starts_with("[") && value.ends_with("]") {
            let parts = Packet::get_elements(&value[1..value.len() - 1])
                .iter()
                .filter(|s| !s.is_empty())
                .map(|part| Packet::new(part))
                .collect();

            Packet::List(parts)
        } else {
            Packet::Integer(value.parse().unwrap())
        }
    }

    fn get_elements(value: &str) -> Vec<&str> {
        let mut elements = Vec::new();
        let mut current_start = 0;
        let mut level = 0;
        for (index, c) in value.char_indices() {
            if c == '[' {
                level += 1;
            } else if c == ']' {
                level -= 1;
            } else if c == ',' && level == 0 {
                elements.push(&value[current_start..index]);
                current_start = index + 1;
            }
        }
        elements.push(&value[current_start..]);

        elements
    }

    fn compare(&self, other: &Packet) -> Ordering {
        match (self, other) {
            (Packet::Integer(first_value), Packet::Integer(second_value)) => {
                first_value.cmp(second_value)
            }
            (Packet::List(first_list), Packet::List(second_list)) => {
                let mut index = 0;
                while index < first_list.len().min(second_list.len()) {
                    let first = &first_list[index];
                    let second = &second_list[index];

                    let result = first.compare(second);
                    if result == Ordering::Greater {
                        return Ordering::Greater;
                    } else if result == Ordering::Less {
                        return Ordering::Less;
                    }
                    index += 1
                }

                first_list.len().cmp(&second_list.len())
            }
            (Packet::List(_), Packet::Integer(second_value)) => {
                let second_wrapped = Packet::new(&format!("[{}]", second_value));

                self.compare(&second_wrapped)
            }
            (Packet::Integer(first_value), Packet::List(_)) => {
                let first_wrapped = Packet::new(&format!("[{}]", first_value));

                first_wrapped.compare(&other)
            }
        }
    }
}

fn main() {
    let input = include_str!("../input");
    let pairs: Vec<_> = input
        .split("\n\n")
        .map(|pair_lines| pair_lines.trim().split_once("\n").unwrap())
        .map(|(first, second)| (Packet::new(first), Packet::new(second)))
        .collect();

    let result = pairs
        .iter()
        .enumerate()
        .filter(|(_index, (first, second))| first.compare(&second) == Ordering::Less)
        .map(|(index, (_, _))| index + 1)
        .sum::<usize>();

    println!("{}", result);
}
