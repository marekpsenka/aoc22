#[derive(Debug)]
struct Tree {
    height: i32,
    is_visible: bool,
}

type Map = Vec::<Vec<Tree>>;

fn mark_visible_if_taller(i: usize, j: usize, tallest: &mut i32, map: &mut Map) {
    let this_tree = &mut map[i][j];
    if this_tree.height > *tallest {
        *tallest = this_tree.height;
        this_tree.is_visible = true;
    }
}

fn main() {
    let mut map = Map::new();

    std::io::stdin()
        .lines()
        .map(|maybe_line| maybe_line.expect("Successful read"))
        .for_each(|line| {
            let row = line
                .chars()
                .map(|c| Tree {
                    height: c
                        .to_digit(10).expect("parsing a digit")
                        .try_into().expect("digit can be converted to i32"),
                    is_visible: false,
                })
                .collect::<Vec<Tree>>();

            map.push(row);
        });

    let width = map.first().expect("At least one row").len();
    let height = map.len();

    for j in 0..width {
        let mut tallest = i32::MIN;
        for i in 0..height {
            mark_visible_if_taller(i, j, &mut tallest, &mut map);
        } 
        tallest = i32::MIN;
        for i in (0..height).rev() {
            mark_visible_if_taller(i, j, &mut tallest, &mut map);
        } 
    }

    for i in 0..height {
        let mut tallest = i32::MIN;
        for j in 0..width {
            mark_visible_if_taller(i, j, &mut tallest, &mut map);
        } 
        tallest = i32::MIN;
        for j in (0..width).rev() {
            mark_visible_if_taller(i, j, &mut tallest, &mut map);
        } 
    }

    let visible_count = map
        .iter()
        .map(|row| row.iter().filter(|t| t.is_visible).count())
        .sum::<usize>();

    println!("{visible_count}");
}
