use std::collections::HashSet;
use itertools::Itertools;

const NEIGHBORS: [(i32, i32, i32); 6] = [(0, 0, -1), (0, 0, 1),
    (1, 0, 0), (0, 1, 0), (-1, 0, 0), (0, -1, 0)];

fn add((x1, y1, z1): &(i32, i32, i32), (x2, y2, z2): &(i32, i32, i32)) -> (i32, i32, i32) {
    (x1 + x2, y1 + y2, z1 + z2)
}

fn main() {
    let points = HashSet::<(i32, i32, i32)>::from_iter(
        std::io::stdin()
            .lines()
            .map(|maybe_line| {
                let line = maybe_line.expect("A line is read");
                let t @ (_, _, _) = line
                    .trim()
                    .split(',')
                    .map(|part| part.parse::<i32>().expect("Parts can be parsed to i32"))
                    .collect_tuple()
                    .expect("Line can be split into three parts");
                t
            }));

    let result = points
        .iter()
        .map(|p| {
            NEIGHBORS
                .iter()
                .filter(|&n| !points.contains(&add(p, n)))
                .count()
        })
        .sum::<usize>();

    println!("{result}")
}
