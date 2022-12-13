use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Element::List(left_sublist) => {
                match other {
                    Element::List(right_sublist) => {
                        let maybe_result = left_sublist.iter()
                            .zip(right_sublist.iter())
                            .map(|(le, re)| le.cmp(re))
                            .find(|&ord| ord != Ordering::Equal);

                        if let Some(result) = maybe_result {
                            result
                        }
                        else {
                            left_sublist.len().cmp(&right_sublist.len())
                        }
                    },
                    Element::Number(right_num) => {
                        self.cmp(&Element::List(vec![Element::Number(*right_num)]))
                    }
                }
            },
            Element::Number(left_num) => {
                match other {
                    Element::List(_) => {
                        Element::List(vec![Element::Number(*left_num)]).cmp(other)
                    },
                    Element::Number(right_num) => {
                        left_num.cmp(right_num)
                    }
                }
            }
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut packets = Vec::<Element>::new();

    std::io::stdin()
        .lines()
        .map(|maybe_line| maybe_line.expect("Successful parse"))
        .group_by(|l| !l.is_empty())
        .into_iter()
        .for_each(|(not_empty, lines)| {
            if not_empty {
                let (left_p, right_p) = lines
                    .collect_tuple()
                    .expect("Input consists of pairs of lines");

                let left_chars = left_p.chars().collect::<Vec<char>>();
                let right_chars = right_p.chars().collect::<Vec<char>>();

                let (left_list, _) = parse_list(&left_chars, 1);
                let (right_list, _) = parse_list(&right_chars, 1);
                
                packets.push(left_list);
                packets.push(right_list);
            }
            else {
            }
        });

    let sep_2 = Element::List(vec![Element::List(vec![Element::Number(2)])]);
    let sep_6 = Element::List(vec![Element::List(vec![Element::Number(6)])]);

    packets.push(sep_2.clone());
    packets.push(sep_6.clone());

    packets.sort();

    let index_of_2 = packets.iter().position(|p| *p == sep_2).expect("Found");
    let index_of_6 = packets.iter().position(|p| *p == sep_6).expect("Found");

    println!("{}", (index_of_2 + 1) * (index_of_6 + 1))
}
