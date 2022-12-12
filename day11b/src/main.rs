#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    modulo: i64,
    if_true: usize,
    if_false: usize,
    touched: u64,
}

impl Monkey {
    fn from_string(input: &str) -> Monkey {
        let lines = input.lines().map(|line| line.trim()).collect::<Vec<_>>();
        let items: Vec<i64> = lines[1]
            .replace("Starting items: ", "")
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();

        let operation = Operation::from_string(lines[2]);
        let modulo = lines[3].replace("Test: divisible by ", "").parse().unwrap();
        let if_true = lines[4]
            .replace("If true: throw to monkey ", "")
            .parse()
            .unwrap();
        let if_false = lines[5]
            .replace("If false: throw to monkey ", "")
            .parse()
            .unwrap();

        Monkey {
            items: items,
            operation: operation,
            modulo: modulo,
            if_true: if_true,
            if_false: if_false,
            touched: 0,
        }
    }

    fn inspect(&mut self, mmc: i64) -> (i64, usize) {
        self.touched += 1;
        let current_item = self.items.pop().unwrap();

        let new_value = self.operation.eval(current_item, mmc);

        let recipient = if new_value % self.modulo as i64 == 0 {
            self.if_true
        } else {
            self.if_false
        };

        (new_value, recipient)
    }
}

#[derive(Debug)]
struct Operation {
    first_operand: String,
    operator: Operator,
    second_operand: String,
}

#[derive(Debug)]
enum Operator {
    ADDITION,
    MULTIPLICATION,
}

impl Operator {
    fn from_string(input: &str) -> Operator {
        match input {
            "+" => Operator::ADDITION,
            "*" => Operator::MULTIPLICATION,
            _ => panic!("Unknown operator: {}", input),
        }
    }
}

impl Operation {
    fn from_string(input: &str) -> Operation {
        let replaced = input.replace("Operation: new = ", "");
        let parts = replaced.split_ascii_whitespace().collect::<Vec<_>>();

        let first_operand = parts[0];
        let operator = Operator::from_string(parts[1]);
        let second_operand = parts[2];

        Operation {
            first_operand: first_operand.to_owned(),
            operator: operator,
            second_operand: second_operand.to_owned(),
        }
    }

    fn eval(&self, old_value: i64, modulo: i64) -> i64 {
        let first = Operation::get_operand_value(&self.first_operand, old_value);
        let second = Operation::get_operand_value(&self.second_operand, old_value);

        match self.operator {
            Operator::ADDITION => ((first + second) as i64) % modulo,
            Operator::MULTIPLICATION => ((first * second) as i64) % modulo,
        }
    }

    fn get_operand_value(value: &str, old_value: i64) -> i64 {
        if value == "old" {
            old_value
        } else {
            value.parse().unwrap()
        }
    }
}

fn main() {
    let mut monkeys: Vec<_> = include_str!("../input")
        .split("\n\n")
        .map(|monkey_info| Monkey::from_string(monkey_info))
        .collect();

    let mmc: i64 = monkeys
        .iter()
        .map(|monkey| monkey.modulo as i64)
        .product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let monkey = monkeys.get_mut(i).unwrap();
                let (item, recipient) = monkey.inspect(mmc);

                let recipient = monkeys.get_mut(recipient).unwrap();

                recipient.items.push(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.touched.cmp(&a.touched));

    let result = monkeys
        .iter()
        .take(2)
        .map(|monkey| monkey.touched)
        .product::<u64>();

    println!("{}", result);
}
