use std::{collections::HashSet};

fn main() {
    let result = std::io::stdin()
        .lines()
        .map(|maybe_line| {
            let line = maybe_line.unwrap();
            let left_compartment: HashSet<char> = line.chars().take(line.len() / 2).collect();
            let right_compartment: HashSet<char> = line.chars().skip(line.len() / 2).collect();
            let i: Vec<&char> = left_compartment.intersection(&right_compartment).collect();
            if i.len() != 1 {
                panic!("Only one type should be found in both compartments");
            }
            if (*i[0]).is_ascii_uppercase() {
                (*i[0] as i32) - (64 - 26)
            }
            else if (*i[0]).is_ascii_lowercase()  {
                (*i[0] as i32) - 96
            }
            else {
                panic!("Unexpected character");
            }
        })
        .sum::<i32>();

    println!("{result}");
}
