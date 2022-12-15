use itertools::Itertools;

type Map = Vec<Vec<bool>>;

fn points_between_nodes(
    &(x1, y1): &(usize, usize),
    &(x2, y2): &(usize, usize),
) -> Vec<(usize, usize)> {
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

fn initialize_blocked(paths: Vec<Vec<(usize, usize)>>, max_x: usize, max_y: usize) -> Map {
    let mut blocked = vec![vec![false; max_x + 1]; max_y + 1];
    for path in paths {
        let mut last = path[0];
        for node in &path[1..] {
            for (x, y) in points_between_nodes(&last, node) {
                blocked[y][x] = true;
            }
            last = *node;
        }
    }

    blocked
}

fn sandfall(
    (x, y): (usize, usize),
    blocked: &Map,
    max_x: usize,
    max_y: usize,
) -> Option<(usize, usize)> {
    for yy in y..max_y + 1 {
        if blocked[yy][x] {
            if x == 0 {
                return None;
            } else if !blocked[yy][x - 1] {
                return sandfall((x - 1, yy), blocked, max_x, max_y);
            } else if x == max_x {
                return None;
            } else if !blocked[yy][x + 1] {
                return sandfall((x + 1, yy), blocked, max_x, max_y);
            } else {
                return Some((x, yy - 1));
            }
        }
    }

    None
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
                        .map(|coord_part| coord_part.parse::<usize>().expect("Succesful parse"))
                        .collect_tuple::<(usize, usize)>()
                        .expect("Coords can be collected to a pair")
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<Vec<(usize, usize)>>>();

    let max_x = rock_paths
        .iter()
        .flatten()
        .max_by_key(|&p| p.0)
        .expect("There is a maximal x coord")
        .0;

    let max_y = rock_paths
        .iter()
        .flatten()
        .max_by_key(|&p| p.1)
        .expect("There is a maximal y coord")
        .1;

    let mut blocked = initialize_blocked(rock_paths, max_x, max_y);

    let mut count = 0;
    while let Some((x, y)) = sandfall((500usize, 0usize), &blocked, max_x, max_y) {
        count += 1;
        blocked[y][x] = true;
    }

    println!("{count}");
}
