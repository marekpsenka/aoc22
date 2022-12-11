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

    let width = 40;
    let height = 6;

    let mut pixels = cycle_values
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            if (i % width).abs_diff(x as usize) <= 1 {
                '#'
            }
            else {
                '.'
            }
        });

    for _ in 0..height {
        for _ in 0..width {
            print!("{}", pixels.next().expect("Value is yielded"));
        }
        println!();
    }
}

