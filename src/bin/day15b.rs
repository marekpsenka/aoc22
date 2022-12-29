use itertools::Itertools;

#[derive(Debug)]
struct Input {
    sensor: (i32, i32),
    beacon: (i32, i32),
}

fn parse_number_after_equals(s: &str) -> i32 {
    let (_, number_part) = s
        .split('=')
        .collect_tuple()
        .expect("Can be split into two parts");

    number_part
        .parse::<i32>()
        .expect("Can be parsed into integer")
}

fn parse_coordinates(s: &str) -> (i32, i32) {
    let (left_part, right_part) = s
        .split(", ")
        .collect_tuple()
        .expect("Can be split into two parts");

    (
        parse_number_after_equals(left_part),
        parse_number_after_equals(right_part),
    )
}

fn manhattan((x1, y1): &(i32, i32), (x2, y2): &(i32, i32)) -> i32 {
    i32::abs(x2 - x1) + i32::abs(y2 - y1)
}

fn boundary((x, y): (i32, i32), r: i32) -> Vec<(i32, i32)> {
    let left = (-r..=r).map(|i| (x - (r - i.abs()), y + i));
    let right = (-r..=r).map(|i| (x + (r - i.abs()), y + i));

    left.chain(right).unique().collect()
}

fn main() {
    let inputs = std::io::stdin()
        .lines()
        .map(|maybe_line| {
            let line = maybe_line.expect("Successful read");
            let (sensor_part, beacon_part) = line
                .split(": ")
                .collect_tuple()
                .expect("Line can be split into two parts");

            Input {
                sensor: parse_coordinates(sensor_part),
                beacon: parse_coordinates(beacon_part),
            }
        })
        .collect::<Vec<Input>>();

    let lost_beacon = inputs
        .iter()
        .find_map(|input| {
            let r = manhattan(&input.sensor, &input.beacon);
            boundary(input.sensor, r + 1).into_iter().find(|bp| {
                inputs
                    .iter()
                    .all(|inp| manhattan(&inp.sensor, bp) > manhattan(&inp.sensor, &inp.beacon)
                                && bp.0 >= 0 && bp.0 <= 4_000_000
                                && bp.1 >= 0 && bp.1 <= 4_000_000)
            })
        })
        .expect("The lost beacon is found");

    println!("{}", lost_beacon.0 * 4_000_000 + lost_beacon.1)
}
