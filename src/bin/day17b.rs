use std::collections::HashSet;
use std::collections::HashMap;

type Chamber = HashSet<(i64, i64)>;

fn add((x1, y1): &(i64, i64), (x2, y2): &(i64, i64)) -> (i64, i64) {
    (x1 + x2, y1 + y2)
}

fn placed_shape<'a>(
    shape: &'a [(i64, i64)],
    origin: &'a (i64, i64),
) -> impl Iterator<Item = (i64, i64)> + 'a {
    shape
        .iter()
        .map(|shape_point: &'a (i64, i64)| add(shape_point, origin))
}

fn simulate_fall(
    shape: &[(i64, i64)],
    spawn_height: i64,
    chamber: &mut Chamber,
    jets: &[char],
    jet_idx: &mut usize,
) -> i64 {
    let mut pos = 2;
    let mut height = spawn_height;

    loop {
        match jets[(*jet_idx) % jets.len()] {
            '>' => {
                let placed_right =
                    placed_shape(shape, &(pos + 1, height)).collect::<HashSet<(i64, i64)>>();

                let (max_x, _) = placed_right
                    .iter()
                    .max_by_key(|&(x, _)| x)
                    .expect("Max found");

                if chamber.intersection(&placed_right).count() == 0 && *max_x < 7 {
                    pos += 1;
                }
            }
            '<' => {
                let placed_left =
                    placed_shape(shape, &(pos - 1, height)).collect::<HashSet<(i64, i64)>>();

                let (min_x, _) = placed_left
                    .iter()
                    .min_by_key(|&(x, _)| x)
                    .expect("Min found");

                if chamber.intersection(&placed_left).count() == 0 && *min_x >= 0 {
                    pos -= 1;
                }
            }
            _ => panic!("Unexpected character in jets"),
        }
        (*jet_idx) += 1;

        let placed_below = placed_shape(shape, &(pos, height - 1)).collect::<HashSet<(i64, i64)>>();

        if chamber.intersection(&placed_below).count() > 0 || height == 0 {
            break;
        }

        height -= 1;
    }

    let resting_shape = placed_shape(shape, &(pos, height)).collect::<Vec<(i64, i64)>>();

    for p in resting_shape.iter() {
        chamber.insert(*p);
    }

    let (_, max_y) = resting_shape
        .into_iter()
        .max_by_key(|&(_, y)| y)
        .expect("Max found");
    max_y
}

fn main() {
    let mut occurence_map = HashMap::<(usize, usize), (usize, usize, i64)>::new();
    let mut chamber = Chamber::new();
    let shapes = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],         // Hor
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], // Plus
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], // Stick
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],         // Vert
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],         // Cube
    ];

    let mut jets_str = String::new();
    std::io::stdin()
        .read_line(&mut jets_str)
        .expect("A line is read");

    let jets = jets_str.chars().collect::<Vec<char>>();
    let mut jet_idx = 0usize;

    let mut max_y = -1;
    let mut fast_forward_occured = false;
    let mut shape_idx = 0;

    while shape_idx < 1_000_000_000_000 {
        let resting_shape_max_y = simulate_fall(
            &shapes[shape_idx % shapes.len()],
            max_y + 4,
            &mut chamber,
            &jets,
            &mut jet_idx,
        );
        if resting_shape_max_y > max_y {
            max_y = resting_shape_max_y;
        }

        if !fast_forward_occured {
            let key = (shape_idx % shapes.len(), jet_idx % jets.len());

            if let Some((2, last_num_shapes, last_max_y)) = occurence_map.get(&key) {
                let shape_num_diff = shape_idx - last_num_shapes;
                let y_diff = max_y - last_max_y;
                let fast_forward_cycles = (1_000_000_000_000 - shape_idx) / shape_num_diff;
                let fast_forward_magnitude = y_diff * (fast_forward_cycles as i64);
                max_y += fast_forward_magnitude;
                chamber = chamber
                    .into_iter()
                    .map(|(x, y)| (x, y + fast_forward_magnitude))
                    .collect();

                shape_idx += fast_forward_cycles * shape_num_diff;

                println!("{} {}", shape_num_diff, y_diff);
                fast_forward_occured = true;
            }

            occurence_map
                .entry(key)
                .and_modify(|(e_ordinal, e_num_shapes, e_max_y)| {
                    *e_ordinal += 1;
                    *e_num_shapes = shape_idx;
                    *e_max_y = max_y
                })
                .or_insert((1, shape_idx, max_y));
        }

        shape_idx += 1;
    }

    println!("{}", max_y + 1);
}
