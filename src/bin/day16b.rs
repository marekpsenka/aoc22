use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::collections::HashMap;

#[derive(Debug)]
struct Input {
    valve: String,
    rate: u32,
    tunnels: Vec<String>,
}

fn parse_name_and_rate(s: &str) -> (String, u32) {
    let (name_part, rate_part) = s
        .split('=')
        .collect_tuple()
        .expect("s can be split into two parts");

    let rate = rate_part
        .parse::<u32>()
        .expect("Rate part can be parsed to u32");

    let pieces = name_part.split_ascii_whitespace().collect_vec();

    (pieces[1].to_string(), rate)
}

fn parse_tunnels(s: &str) -> Vec<String> {
    let mut parts = s.split(", ").map(|slice| slice.to_string()).collect_vec();

    let second = parts[0].pop().expect("character");
    let first = parts[0].pop().expect("character");
    parts[0] = format!("{}{}", first, second);
    parts
}

fn calculate_distances(inputs: &Vec<Input>, index: &HashMap<String, usize>) -> Vec<Vec<u32>> {
    let mut distances = vec![vec![u32::MAX; inputs.len()]; inputs.len()];
    for input in inputs {
        for tunnel in input.tunnels.iter() {
            let i = index[&input.valve];
            distances[i][index[tunnel]] = 1;
            distances[i][i] = 0;
        }
    }

    for k in 0..inputs.len() {
        for i in 0..inputs.len() {
            for j in 0..inputs.len() {
                let new_dist = distances[i][k]
                    .checked_add(distances[k][j])
                    .unwrap_or(u32::MAX);
                if distances[i][j] > new_dist {
                    distances[i][j] = new_dist
                }
            }
        }
    }

    distances
}

fn prepare_search(inputs: &[Input], distances: &[Vec<u32>]) -> (Vec<u32>, Vec<Vec<u32>>) {
    let mask = inputs
        .iter()
        .map(|input| input.rate > 0 || input.valve == "AA");
    let trimmed = distances
        .iter()
        .zip(mask.clone())
        .filter_map(|(row, keep)| {
            if keep {
                let trimmed_row = row
                    .iter()
                    .zip(mask.clone())
                    .filter_map(|(element, keep)| if keep { Some(*element) } else { None })
                    .collect::<Vec<u32>>();
                Some(trimmed_row)
            } else {
                None
            }
        })
        .collect();

    let trimmed_rates = inputs
        .iter()
        .zip(mask)
        .filter_map(|(input, keep)| if keep { Some(input.rate) } else { None })
        .collect::<Vec<u32>>();

    (trimmed_rates, trimmed)
}

#[derive(Hash, PartialEq, Eq)]
struct State {
    my_node: usize,
    el_node: usize,
    not_visited: Vec<usize>,
    my_remaining: Option<u32>,
    el_remaining: Option<u32>,
    released: u32,
}

fn send_me(
    s: &State,
    distances: &[Vec<u32>],
    rates: &[u32],
    candidate: &mut u32,
    q: &mut PriorityQueue<State, u32>,
) {
    let my_remaining_value = s.my_remaining.expect("Still some time remaining");
    for &i in s.not_visited.iter() {
        if let Some(my_remaining) = my_remaining_value.checked_sub(distances[s.my_node][i] + 1) {
            let mut not_visited = s.not_visited.clone();
            let (position, _) = not_visited
                .iter()
                .find_position(|&j| *j == i)
                .expect("Found");
            not_visited.remove(position);
            let released = s.released + rates[i] * my_remaining;
            let potential = not_visited
                .iter()
                .map(|j| rates[*j] * my_remaining)
                .sum::<u32>();

            if potential + released > *candidate {
                q.push(
                    State {
                        my_node: i,
                        el_node: s.el_node,
                        not_visited,
                        my_remaining: Some(my_remaining),
                        el_remaining: s.el_remaining,
                        released,
                    },
                    released,
                );
            }
        } else if s.el_remaining.is_none() {
            if s.released > *candidate {
                *candidate = s.released;
            }
        }
        else {
            q.push(
                State {
                    my_node: s.my_node,
                    el_node: s.el_node,
                    not_visited: s.not_visited.clone(),
                    my_remaining: None,
                    el_remaining: s.el_remaining,
                    released: s.released,
                },
                s.released,
            );
        }
    }
}

fn send_el(
    s: &State,
    distances: &[Vec<u32>],
    rates: &[u32],
    candidate: &mut u32,
    q: &mut PriorityQueue<State, u32>,
) {
    let el_remaining_value = s.el_remaining.expect("Still some time remaining");
    for &i in s.not_visited.iter() {
        if let Some(el_remaining) = el_remaining_value.checked_sub(distances[s.el_node][i] + 1) {
            let mut not_visited = s.not_visited.clone();
            let (position, _) = not_visited
                .iter()
                .find_position(|&j| *j == i)
                .expect("Found");
            not_visited.remove(position);
            let released = s.released + rates[i] * el_remaining;
            let potential = not_visited
                .iter()
                .map(|j| rates[*j] * el_remaining)
                .sum::<u32>();

            if potential + released > *candidate {
                q.push(
                    State {
                        my_node: s.my_node,
                        el_node: i,
                        not_visited,
                        my_remaining: s.my_remaining,
                        el_remaining: Some(el_remaining),
                        released,
                    },
                    released,
                );
            }
        } else if s.my_remaining.is_none() {
            if s.released > *candidate {
                *candidate = s.released;
            }
        }
        else {
            q.push(
                State {
                    my_node: s.my_node,
                    el_node: s.el_node,
                    not_visited: s.not_visited.clone(),
                    my_remaining: s.my_remaining,
                    el_remaining: None,
                    released: s.released,
                },
                s.released,
            );
        }
    }
}

fn main() {
    let inputs = std::io::stdin()
        .lines()
        .map(|maybe_line| {
            let line = maybe_line.expect("A line is read");
            let (left_part, right_part) = line
                .split(';')
                .collect_tuple()
                .expect("Line can be split into two parts");

            let (valve, rate) = parse_name_and_rate(left_part);
            let tunnels = parse_tunnels(right_part);
            Input {
                valve,
                rate,
                tunnels,
            }
        })
        .sorted_by_key(|input| input.valve.clone())
        .collect::<Vec<Input>>();

    let index = HashMap::<String, usize>::from_iter(
        inputs
            .iter()
            .map(|input| input.valve.clone())
            .enumerate()
            .map(|(x, y)| (y, x)),
    );

    let all_distances = calculate_distances(&inputs, &index);
    let (rates, distances) = prepare_search(&inputs, &all_distances);

    // Tracking at every level:
    // - Remaining time
    // - Visited nodes
    // - Released Pressure
    //
    // Tracking globally:
    // - Candidate
    //
    // Trimming:
    // - Abandon search if RT * sum(rate(!visited)) + RP <= Candidate

    let mut q = PriorityQueue::<State, u32>::new();

    q.push(
        State {
            my_node: 0,
            el_node: 0,
            not_visited: (1..rates.len()).collect(),
            my_remaining: Some(26),
            el_remaining: Some(26),
            released: 0,
        },
        u32::MAX,
    );
    let mut candidate = 0u32;

    while let Some((s, _)) = q.pop() {
        if s.not_visited.is_empty() {
            if s.released > candidate {
                candidate = s.released;
            }
            continue;
        }
        else if let Some(my_remaining_value) = s.my_remaining {
            if let Some(el_remaining_value) = s.el_remaining {
                match my_remaining_value.cmp(&el_remaining_value) {
                    std::cmp::Ordering::Less => send_el(&s, &distances, &rates, &mut candidate, &mut q),
                    _ => send_me(&s, &distances, &rates, &mut candidate, &mut q)
                }
            }
            else {
                send_me(&s, &distances, &rates, &mut candidate, &mut q)
            }
        }
        else if s.el_remaining.is_some() {
            send_el(&s, &distances, &rates, &mut candidate, &mut q)
        }
    }

    println!("{:?}", candidate);
}
