use itertools::Itertools;
use std::collections::HashSet;

type Blocked = HashSet<(i32, i32)>;

fn points_between_nodes(
    &(x1, y1): &(i32, i32),
    &(x2, y2): &(i32, i32),
) -> Vec<(i32, i32)> {
    if x1 == x2 {
        match y1.cmp(&y2) {
            std::cmp::Ordering::Less => (y1..y2 + 1).map(|y| (x1, y)).collect(),
            std::cmp::Ordering::Equal => panic!("Unexpected"),
            std::cmp::Ordering::Greater => (y2..y1 + 1).map(|y| (x1, y)).collect(),
        }
    } else if y1 == y2 {
        match x1.cmp(&x2) {
            std::cmp::Ordering::Less => (x1..x2 + 1).map(|x| (x, y1)).collect(),
            std::cmp::Ordering::Equal => panic!("Unexpected"),
            std::cmp::Ordering::Greater => (x2..x1 + 1).map(|x| (x, y1)).collect(),
        }
    } else {
        panic!("No coordinates are common")
    }
}

fn initialize_blocked(paths: Vec<Vec<(i32, i32)>>) -> Blocked {
    let mut blocked = Blocked::new();
    for path in paths {
        let mut last = path[0];
        for node in &path[1..] {
            for p in points_between_nodes(&last, node) {
                blocked.insert(p);
            }
            last = *node;
        }
    }

    blocked
}

fn sandfall(
    (x, y): (i32, i32),
    blocked: &Blocked,
    max_y: i32,
) -> (i32, i32) {
    let (mut xx, mut yy) = (x, y);
    loop {
        if yy == max_y + 1 {
            return (xx, yy);
        } else if !blocked.contains(&(xx, yy + 1)) {
            yy += 1;
        } else if !blocked.contains(&(xx - 1, yy + 1)) {
            xx -= 1;
            yy += 1;
        } else if !blocked.contains(&(xx + 1, yy + 1)) {
            xx += 1;
            yy += 1;
        } else {
            return (xx, yy);
        }
    }
}

fn main() {
    let rock_paths = std::io::stdin()
        .lines()
        .map(|maybe_line| {
            maybe_line
                .expect("Successful read")
                .split(" -> ")
                .map(|coords_part| {
                    coords_part
                        .split(',')
                        .map(|coord_part| coord_part.parse::<i32>().expect("Succesful parse"))
                        .collect_tuple::<(i32, i32)>()
                        .expect("Coords can be collected to a pair")
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .collect::<Vec<Vec<(i32, i32)>>>();

    let max_y = rock_paths
        .iter()
        .flatten()
        .max_by_key(|&p| p.1)
        .expect("There is a maximal y coord")
        .1;

    let mut blocked = initialize_blocked(rock_paths);

    let mut count = 0;
    let inlet = (500i32, 0i32);
    loop {
        let grain = sandfall(inlet, &blocked, max_y);
        count += 1;
        blocked.insert(grain);
        if grain == inlet {
            break;
        }
        
    }

    println!("{count}");
}
