use itertools::Itertools;
use std::cmp::Ordering;

enum Element {
    Number(i32),
    List(Vec<Element>)
}

fn parse_list(cs: &[char], idx: usize) -> (Element, usize) {
    let mut elements = Vec::<Element>::new();
    let mut i = idx;
    loop {
        match cs[i] {
            d if d.is_ascii_digit() => {
                let mut digits = vec![d];
                let mut j = i + 1;
                while cs[j].is_ascii_digit() {
                    digits.push(cs[j]);
                    j += 1;
                }
                elements.push(Element::Number(digits
                    .into_iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .expect("Successful parse")));
                i = j
            }
            ']' => break,
            ',' => i += 1,
            '[' => {
                let (sub_list, new_i) = parse_list(cs, i + 1);
                i = new_i;
                elements.push(sub_list);
            }
            _ => panic!("Unexpected character")
        }
    } 

    (Element::List(elements), i + 1)
}

fn compare(left_element: &Element, right_element: &Element) -> Ordering {
    match left_element {
        Element::List(left_sublist) => {
            match right_element {
                Element::List(right_sublist) => {
                    let maybe_result = left_sublist.iter()
                        .zip(right_sublist.iter())
                        .map(|(le, re)| compare(le, re))
                        .find(|&ord| ord != Ordering::Equal);

                    if let Some(result) = maybe_result {
                        result
                    }
                    else {
                        left_sublist.len().cmp(&right_sublist.len())
                    }
                },
                Element::Number(right_num) => {
                    compare(left_element,
                         &Element::List(vec![Element::Number(*right_num)]))
                }
            }
        },
        Element::Number(left_num) => {
            match right_element {
                Element::List(_) => {
                    compare(&Element::List(vec![Element::Number(*left_num)]), right_element)
                },
                Element::Number(right_num) => {
                    left_num.cmp(right_num)
                }
            }
        }
    }
}

fn main() {
    let sum = std::io::stdin()
        .lines()
        .map(|maybe_line| maybe_line.expect("Successful parse"))
        .group_by(|l| !l.is_empty())
        .into_iter()
        .filter_map(|(not_empty, lines)| {
            if not_empty {
                let (left_p, right_p) = lines
                    .collect_tuple()
                    .expect("Input consists of pairs of lines");

                let left_chars = left_p.chars().collect::<Vec<char>>();
                let right_chars = right_p.chars().collect::<Vec<char>>();

                let (left_list, _) = parse_list(&left_chars, 1);
                let (right_list, _) = parse_list(&right_chars, 1);
                
                Some((left_list, right_list))
            }
            else {
                None
            }
        })
        .enumerate()
        .filter_map(|(index, (left_list, right_list))| {
            if compare(&left_list, &right_list) != Ordering::Greater {
                Some(index + 1)
            }
            else {
                None
            }
        })
        .sum::<usize>();

    println!("{}", sum);
}
