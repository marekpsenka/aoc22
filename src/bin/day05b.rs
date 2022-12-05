use itertools::Itertools;

struct Instruction {
    count: i32,
    from: i32,
    to: i32
}

fn parse_instruction(s: &String) -> Instruction {
    let (count, from, to) = s
        .split_ascii_whitespace()
        .filter_map(|part| {
            if let Ok(i) = part.parse::<i32>() {
                Some(i)
            }
            else {
                None
            }
        })
        .collect_tuple()
        .unwrap();

    Instruction { count, from, to }
}

impl Instruction {
    fn apply(self, stacks: &mut [Vec<char>]) {
        let moved : Vec<char> = (0..self.count)
            .into_iter()
            .map(|_| stacks[self.from as usize - 1].pop().unwrap())
            .collect();

        for m in moved.into_iter().rev() {
            stacks[self.to as usize - 1].push(m);
        }
    }
}

fn main() {
    let all_lines: Vec<String> =
         std::io::stdin()
            .lines() 
            .map(|maybe_line| maybe_line.unwrap())
            .collect();

    let stack_line_count = all_lines
        .iter()
        .take_while(|l| !l.is_empty())
        .count();

    let stack_count = all_lines[stack_line_count - 1]
        .as_str().split_whitespace().count();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    for _ in 0..stack_count {
        stacks.push(Vec::new())
    }

    for line in all_lines.iter().take(stack_line_count).rev() {
        let cs: Vec<char> = (*line).chars().collect();
        for i in 0..stack_count {
            let c = cs[4 * i + 1];
            if c.is_ascii_alphabetic() {
                stacks[i].push(c);
            }
        }
    }

    let instructions = all_lines
        .iter()
        .skip(stack_line_count + 1)
        .map(parse_instruction);

    instructions.for_each(|ins| ins.apply(&mut stacks));

    let result = stacks
        .iter()
        .map(|s| *(s.last().unwrap()))
        .collect::<String>();

    println!("{result}");
}
