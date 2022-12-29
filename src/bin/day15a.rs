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

fn merge_two((x1, y1): &(i32, i32), (x2, y2): &(i32, i32)) -> Option<(i32, i32)> {
    if y1 < x2 {
        None
    } else if y1 >= y2 {
        Some((*x1, *y1))
    } else {
        Some((*x1, *y2))
    }
}

fn point_count((x, y): &(i32, i32)) -> u32 {
    y.abs_diff(*x) + 1
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

    let row = 2_000_000;
    let mut intervals = inputs
        .iter()
        .filter_map(|input| {
            let radius = manhattan(&input.sensor, &input.beacon);
            let y_diff = i32::abs(row - input.sensor.1);
            let x_diff = radius - y_diff;
            if y_diff < radius {
                Some((input.sensor.0 - x_diff, input.sensor.0 + x_diff))
            }
            else {
                None
            }
        })
        .collect::<Vec<(i32, i32)>>();

    intervals.sort_by_key(|p| p.0);
    let mut last = intervals[0];
    let mut count = 0u32;

    for interval in intervals.iter().skip(1) {
        match merge_two(&last, interval) {
            Some(merged) => {
                last = merged
            },
            None => {
                count += point_count(&last);
                last = *interval;
            },
        }
    }

    count += point_count(&last);

    let beacons_on_row = inputs
        .iter()
        .filter(|input| input.beacon.1 == row)
        .map(|input| input.beacon)
        .unique()
        .count();

    println!("{}", count - beacons_on_row as u32)
}
