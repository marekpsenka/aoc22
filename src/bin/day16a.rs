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
    current_node: usize,
    not_visited: Vec<usize>,
    remaining: u32,
    released: u32,
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
            current_node: 0,
            not_visited: (1..rates.len()).collect(),
            remaining: 30,
            released: 0,
        },
        30 * rates.iter().sum::<u32>(),
    );
    let mut candidate = 0u32;

    while let Some((s, _)) = q.pop() {
        if s.not_visited.is_empty() {
            if s.released > candidate {
                candidate = s.released;
            }
            continue;
        }
        for &i in s.not_visited.iter() {
            if let Some(remaining) = s.remaining.checked_sub(distances[s.current_node][i] + 1) {
                let mut not_visited = s.not_visited.clone();
                let (position, _) = not_visited.iter().find_position(|&j| *j == i).expect("Found");
                not_visited.remove(position);
                let released = s.released + rates[i] * remaining;
                let potential = not_visited.iter().map(|j| rates[*j] * remaining).sum::<u32>();

                if potential + released > candidate {
                    q.push(State { current_node: i, not_visited, remaining, released }, potential);
                }
                else {
                    println!(".");
                }
            }
            else if s.released > candidate {
                candidate = s.released;
            }
        }
    }

    println!("{:?}", candidate);
}
