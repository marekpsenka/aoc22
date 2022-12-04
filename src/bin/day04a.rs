use itertools::Itertools;

type Assignment = (i32, i32);

fn parse_assignment(s: &str) -> Assignment {
    let (sp1, sp2) = s.split('-').collect_tuple().unwrap();

    (sp1.parse::<i32>().unwrap(), sp2.parse::<i32>().unwrap())
}

fn is_inside(some: &Assignment, other: &Assignment) -> bool {
    let (l1, u1) = some;
    let (l2, u2) = other;

    l1 >= l2 && u1 <= u2
}

fn main() {
    let result = std::io::stdin()
        .lines()
        .map(|maybe_line| maybe_line.unwrap())
        .filter(
            |line| {
                let (p1, p2) = line.split(',').collect_tuple().unwrap();
                let a1 = parse_assignment(p1);
                let a2 = parse_assignment(p2);

                is_inside(&a1, &a2) || is_inside(&a2, &a1)
            }
        )
        .count();

    println!("{result}");
}
