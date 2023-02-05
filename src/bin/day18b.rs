use itertools::Itertools;
use std::collections::{HashSet};
use priority_queue::PriorityQueue;

type PointSet = HashSet<(i32, i32, i32)>;

const NEIGHBORS: [(i32, i32, i32); 6] = [
    (0, 0, -1),
    (0, 0, 1),
    (1, 0, 0),
    (0, 1, 0),
    (-1, 0, 0),
    (0, -1, 0),
];

fn add((x1, y1, z1): &(i32, i32, i32), (x2, y2, z2): &(i32, i32, i32)) -> (i32, i32, i32) {
    (x1 + x2, y1 + y2, z1 + z2)
}

fn within_bounds(
    (x, y, z): &(i32, i32, i32),
    ((min_x, max_x), (min_y, max_y), (min_z, max_z)): &Bounds,
) -> bool {
    x >= min_x && x <= max_x && y >= min_y && y <= max_y && z >= min_z && z <= max_z
}

type Bounds = ((i32, i32), (i32, i32), (i32, i32));

fn get_bounds(points: &PointSet) -> Bounds {
    (
        (
            points.iter().min_by_key(|&p| p.0).expect("Min is found").0,
            points.iter().max_by_key(|&p| p.0).expect("Max is found").0,
        ),
        (
            points.iter().min_by_key(|&p| p.1).expect("Min is found").1,
            points.iter().max_by_key(|&p| p.1).expect("Max is found").1,
        ),
        (
            points.iter().min_by_key(|&p| p.2).expect("Min is found").2,
            points.iter().max_by_key(|&p| p.2).expect("Max is found").2,
        ),
    )
}

fn search(
    start: &(i32, i32, i32),
    bounds: &Bounds,
    droplet: &PointSet,
    pockets: &mut PointSet,
    true_exterior: &mut PointSet,
) {
    let mut queue = PriorityQueue::<(i32, i32, i32), u32>::new();
    let mut path = PointSet::new();
    queue.push_increase(*start, u32::MAX);

    let mut reached_true_exterior = false;
    while let Some((p, _)) = queue.pop() {
        path.insert(p);
        NEIGHBORS
            .iter()
            .map(|n| add(&p, n))
            .for_each(|np| {
                if droplet.contains(&np) || pockets.contains(&np) || path.contains(&np) {
                }
                else if true_exterior.contains(&np) || !within_bounds(&np, bounds) {
                    reached_true_exterior = true;
                }
                else {
                    queue.push_increase(np, u32::MAX);
                }
            });

        if reached_true_exterior {
            break;
        }
    }

    if reached_true_exterior {
        true_exterior.extend(path.drain());
    } else {
        pockets.extend(path.drain());
    }
}

fn main() {
    let droplet = PointSet::from_iter(std::io::stdin().lines().map(|maybe_line| {
        let line = maybe_line.expect("A line is read");
        let t @ (_, _, _) = line
            .trim()
            .split(',')
            .map(|part| part.parse::<i32>().expect("Parts can be parsed to i32"))
            .collect_tuple()
            .expect("Line can be split into three parts");
        t
    }));

    let exterior = droplet
        .iter()
        .flat_map(|p| {
            NEIGHBORS
                .iter()
                .map(|n| add(p, n))
                .filter(|np| !droplet.contains(np))
        })
        .collect::<PointSet>();

    let bounds = get_bounds(&droplet);

    let mut true_exterior = PointSet::new();
    let mut pockets = PointSet::new();

    for p in exterior.iter() {
        search(p, &bounds, &droplet, &mut pockets, &mut true_exterior)
    }

    let result = droplet
        .iter()
        .map(|p| {
            NEIGHBORS
                .iter()
                .filter(|&n| true_exterior.contains(&add(p, n)))
                .count()
        })
        .sum::<usize>();
    
    println!("{result}");
}
