use itertools::Itertools;

struct Monkey {
    items: Vec<i32>,
    divisor: i32,
    operation: Box<dyn Fn(i32) -> i32>,
    true_target: usize,
    false_target: usize
}

fn parse_operation(s: &str) -> Box<dyn Fn(i32) -> i32> {
    let (_, op_part, right_part) = s
        .split_ascii_whitespace()
        .collect_tuple()
        .expect("Operation part can be split into three parts");

    if let Ok(num) = right_part.parse::<i32>() {
        match op_part {
            "*" => Box::new(move |x| x * num),
            "+" => Box::new(move |x| x + num),
            _ => panic!("Unexpected operation symbol")
        }
    }
    else {
        match op_part {
            "*" => Box::new(|x| x * x),
            "+" => Box::new(|x| x + x),
            _ => panic!("Unexpected operation symbol")
        }
    }
}

fn parse_usize_from_end_of_line(s: &str) -> usize {
    s.split_ascii_whitespace()
        .last()
        .expect("Split line has a last part")
        .parse::<usize>()
        .expect("Last part can be parsed to usize")
}

fn main() {
    let mut monkeys = std::io::stdin()
        .lines()
        .map(|maybe_line| maybe_line.expect("Successful read"))
        .group_by(|line| !line.is_empty())
        .into_iter()
        .filter_map(|(group_condition, grouped_lines)| {
            if group_condition {
                Some(grouped_lines)
            }
            else {
                None
            }
        })
        .map(|mut lines| {
            let _ = lines.next();

            let items_line = lines.next().expect("Line with items is yielded");
            let (_, item_list_part) = items_line
                .split(": ")
                .collect_tuple()
                .expect("Items line can be broken into two parts");

            let items = item_list_part
                .split(", ")
                .map(|part| {
                    part.parse::<i32>().expect("Parts of items can be parsed")
                })
                .collect::<Vec<i32>>();

            let operation_line = lines.next().expect("Line with operation is yielded");
            let (_, op_part) = operation_line
                .split("= ")
                .collect_tuple()
                .expect("Operation line can be broken into two parts");

            let operation = parse_operation(op_part);

            let divisor = parse_usize_from_end_of_line(
                lines.next().expect("Line is yielded").as_str()
            ) as i32;

            let true_target = parse_usize_from_end_of_line(
                lines.next().expect("Line is yielded").as_str()
            );

            let false_target = parse_usize_from_end_of_line(
                lines.next().expect("Line is yielded").as_str()
            );

            Monkey { items, operation, divisor, true_target, false_target }
        })
        .collect::<Vec<Monkey>>();

    let mut counts = vec![0usize; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            counts[i] += monkeys[i].items.len();
            let thrown = monkeys[i].items
                .drain(..)
                .collect::<Vec<i32>>();

            thrown
                .into_iter()
                .for_each(|item| {
                    let updated_worry = (monkeys[i].operation)(item);
                    let decreased_worry = updated_worry / 3;
                    let target = if decreased_worry % monkeys[i].divisor == 0 {
                        monkeys[i].true_target
                    }
                    else {
                        monkeys[i].false_target
                    };

                    monkeys[target].items.push(decreased_worry);
                });
        }
    }

    let (max, next_to_max) = counts
        .into_iter()
        .sorted()
        .rev()
        .take(2)
        .collect_tuple()
        .expect("counts.len() >= 2");

    println!("{}", max * next_to_max)
}
