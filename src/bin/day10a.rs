use itertools::Itertools;

fn main() {
    let mut x = 1i32;
    let mut cycle_values = Vec::<i32>::new();

    for maybe_line in std::io::stdin().lines() {
        let line = maybe_line.expect("Successful read");
        if line.starts_with("noop") {
            cycle_values.push(x);
        }
        else {
            let (_, value_str) = line
                .split_ascii_whitespace()
                .collect_tuple()
                .expect("Pair");

            let value = value_str.parse::<i32>().expect("Successful parse");
            cycle_values.push(x);
            cycle_values.push(x);
            x += value;
        }
    }

    let result = vec![20, 60, 100, 140, 180, 220]
        .iter()
        .map(|&i| {
            i * cycle_values[i - 1] as usize
        })
        .sum::<usize>();

    println!("{result}");
}
