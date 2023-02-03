use std::{collections::HashSet, iter::Cycle, str::Chars};

type Chamber = HashSet<(i32, i32)>;

fn add((x1, y1): &(i32, i32), (x2, y2): &(i32, i32)) -> (i32, i32) {
    (x1 + x2, y1 + y2)
}

fn placed_shape<'a>(
    shape: &'a [(i32, i32)],
    origin: &'a (i32, i32),
) -> impl Iterator<Item = (i32, i32)> + 'a {
    shape.iter().map(|shape_point: &'a(i32, i32)| add(shape_point, origin))
}

fn simulate_fall(shape: &[(i32, i32)], spawn_height: i32, chamber: &mut Chamber,
    jets: &mut Cycle<Chars>) -> i32
{
    let mut pos = 2;
    let mut height = spawn_height;

    loop {
        match jets.next().expect("Jets cycle never ends") {
            '>' => {
                let placed_right = placed_shape(shape, &(pos + 1, height))
                    .collect::<HashSet<(i32, i32)>>();

                let (max_x, _) = placed_right.iter().max_by_key(|&(x, _)| x).expect("Max found");

                if chamber.intersection(&placed_right).count() == 0 && *max_x < 7 {
                        pos += 1;
                }
            }
            '<' => {
                let placed_left = placed_shape(shape, &(pos - 1, height))
                    .collect::<HashSet<(i32, i32)>>();

                let (min_x, _) = placed_left.iter().min_by_key(|&(x, _)| x).expect("Min found");

                if chamber.intersection(&placed_left).count() == 0 && *min_x >= 0 {
                        pos -= 1;
                }
            }
            _ => panic!("Unexpected character in jets")
        }

        let placed_below = placed_shape(shape, &(pos, height - 1))
            .collect::<HashSet<(i32, i32)>>();

        if chamber.intersection(&placed_below).count() > 0 || height == 0 {
            break;
        }

        height -= 1;
    }

    let resting_shape = placed_shape(shape, &(pos, height))
        .collect::<Vec<(i32, i32)>>();

    for p in resting_shape.iter() {
        chamber.insert(*p);
    }

    let (_, max_y) = resting_shape.into_iter().max_by_key(|&(_, y)| y).expect("Max found");
    max_y
}


fn main() {
    let mut chamber = Chamber::new();
    let shapes = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],         // Hor
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], // Plus
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], // Stick
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],         // Vert
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],         // Cube
    ];

    let mut shapes_cycle = shapes.iter().cycle();

    let mut jets_str = String::new();
    std::io::stdin().read_line(&mut jets_str).expect("A line is read");

    let mut jets = jets_str.chars().cycle();

    let mut max_y = -1;

    for _ in 0..2022 {
        let shape = shapes_cycle.next().expect("Shapes cycle never ends");
        let resting_shape_max_y = simulate_fall(shape, max_y + 4, &mut chamber, &mut jets);
        if resting_shape_max_y > max_y {
            max_y = resting_shape_max_y;
        }
    }

    println!("{}", max_y + 1);
}
