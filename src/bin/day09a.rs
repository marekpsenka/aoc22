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

fn main() {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut tail = (0i32, 0i32);
    let mut head = (0i32, 0i32);
    visited.insert(tail);
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
                move_point(&mut head, dir);
                let (hx, hy) = head;
                if sup_dist(&head, &tail) > 1 {
                    match dir {
                        "U" => tail = (hx, hy - 1),
                        "D" => tail = (hx, hy + 1),
                        "L" => tail = (hx + 1, hy),
                        "R" => tail = (hx - 1, hy),
                        _ => panic!("Unexpected direction")
                    }
                    visited.insert(tail);
                }
            }
        });

    println!("{}", visited.len())
}
