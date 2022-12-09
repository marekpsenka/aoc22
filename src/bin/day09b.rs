use std::collections::HashSet;
use itertools::Itertools;

fn sup_dist((ax, ay): &(i32, i32), (bx, by): &(i32, i32)) -> i32 {
    i32::max(i32::abs(ax - bx), i32::abs(ay - by))
}

fn move_point((px, py): &mut(i32, i32), dir: &str) {
    match dir {
        "U" => *py += 1,
        "D" => *py -= 1,
        "L" => *px -= 1,
        "R" => *px += 1,
        _ => panic!("Unexpected direction")
    }
}

fn new_pos(moved: &(i32, i32), next: &(i32, i32)) -> (i32, i32) {
    let mut new_pos = *next;
    if sup_dist(moved, next) > 1 {
        if moved.0 > next.0 { new_pos.0 += 1; }
        if moved.0 < next.0 { new_pos.0 -= 1; }
        if moved.1 > next.1 { new_pos.1 += 1; }
        if moved.1 < next.1 { new_pos.1 -= 1; }
    }

    new_pos
}

fn main() {
    let knot_count = 10;
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut knots = std::iter::repeat((0i32, 0i32))
        .take(knot_count)
        .collect::<Vec<(i32, i32)>>();

    visited.insert((0i32, 0i32));

    std::io::stdin()
        .lines()
        .map(|maybe_line| maybe_line.expect("Successful read"))
        .for_each(|line| {
            let (dir, dist_str) = line
                .split_ascii_whitespace()
                .collect_tuple()
                .expect("Line contains direction and distance parts");

            let dist = dist_str.parse::<i32>().expect("Distance part can be parsed to i32");

            for _ in 0..dist {
                move_point(&mut knots[0], dir);
                for i in 1..knot_count {
                    knots[i] = new_pos(&knots[i - 1], &knots[i]);

                }

                visited.insert(*(knots.last().expect("Pair is found")));
            }
        });

    println!("{}", visited.len())
}
