use std::{collections::HashSet};

use itertools::Itertools;

fn get_priority(c: char) -> i32 {
    if c.is_ascii_uppercase() {
        (c as i32) - (64 - 26)
    }
    else if c.is_ascii_lowercase()  {
        (c as i32) - 96
    }
    else {
        panic!("Unexpected character");
    }
}


fn main() {
    let result = std::io::stdin()
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let bags = chunk
                .map(|maybe_line| {
                    let line = maybe_line.unwrap();
                    line.chars().collect::<HashSet<char>>()
                })
                .collect::<Vec<HashSet<char>>>();

            if bags.len() != 3 {
                panic!("Unexpected number of bags");
            }
            
            let i1: HashSet<char> = bags[0].intersection(&bags[1]).into_iter().copied().collect();
            let i2: Vec<&char> = i1.intersection(&bags[2]).collect();
            if i2.len() != 1 {
                panic!("Only one type should be found in all three compartments");
            }
            get_priority(*i2[0])
        })
        .sum::<i32>();

    println!("{result}");
}
