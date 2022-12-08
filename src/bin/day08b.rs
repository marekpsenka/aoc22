#[derive(Debug)]
struct Tree {
    height: i32,
}

type Map = Vec::<Vec<Tree>>;

fn get_scenic_score(width: usize, height: usize, i: usize, j: usize, map: &mut Map) -> usize {
    let origin_height = map[i][j].height;
    let mut score = 1usize;

    let mut direction_score = 0usize;
    for ii in (0..i).rev() {
        direction_score += 1;
        if map[ii][j].height >= origin_height {
            break;
        }
    } 

    score *= direction_score;
    direction_score = 0;

    for ii in (i + 1)..height {
        direction_score += 1;
        if map[ii][j].height >= origin_height {
            break;
        }
    } 

    score *= direction_score;
    direction_score = 0;

    for jj in (0..j).rev() {
        direction_score += 1;
        if map[i][jj].height >= origin_height {
            break;
        }
    } 

    score *= direction_score;
    direction_score = 0;

    for jj in (j + 1)..width {
        direction_score += 1;
        if map[i][jj].height >= origin_height {
            break;
        }
    } 
    score *= direction_score;
    score
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
                })
                .collect::<Vec<Tree>>();

            map.push(row);
        });

    let width = map.first().expect("At least one row").len();
    let height = map.len();

    let mut highest_scenic_score = usize::MIN;

    println!("{}", get_scenic_score(width, height, 3, 2, &mut map));

    for i in 0..height {
        for j in 0..width {
            let score = get_scenic_score(width, height, i, j, &mut map);
            if score > highest_scenic_score {
                highest_scenic_score = score;
            }
        }
    }
    println!("{highest_scenic_score}");
}
