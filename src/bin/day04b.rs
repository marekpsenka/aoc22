use itertools::Itertools;

type Assignment = (i32, i32);

fn parse_assignment(s: &str) -> Assignment {
    let (sp1, sp2) = s.split('-').collect_tuple().unwrap();

    (sp1.parse::<i32>().unwrap(), sp2.parse::<i32>().unwrap())
}

fn do_overlap(some: &Assignment, other: &Assignment) -> bool {
    let (l1, u1) = some;
    let (l2, u2) = other;

    u1 >= l2 && l1 <= u2
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

                do_overlap(&a1, &a2)
            }
        )
        .count();

    println!("{result}");
}