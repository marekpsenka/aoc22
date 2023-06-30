use std::collections::HashMap;

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_cw(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn rotate_ccw(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn eval(self) -> usize {
        match self {
            Direction::Up => 3,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Move(usize),
    TurnLeft,
    TurnRight,
}

type Tile = Vec<Vec<char>>;
type Board = Vec<Vec<Option<Tile>>>;
type EdgeMap = HashMap<((usize, usize), Direction), (usize, usize)>;

fn copy_tile(lines: &[Vec<char>], kk: usize, ll: usize, a: usize) -> Tile {
    let mut tile = vec![vec!['x'; a]; a];

    for y in 0..a {
        tile[y].clone_from_slice(&lines[ll * a + y][(kk * a)..(kk + 1) * a])
    }

    tile
}

fn board_from_lines(lines: &[Vec<char>], k: usize, l: usize, a: usize) -> Board {
    let mut board: Board = vec![vec![None; k]; l];

    for ll in 0..l {
        for kk in 0..k {
            if kk * a >= lines[ll * a].len() {
                continue;
            }

            if lines[ll * a][kk * a].is_whitespace() {
                continue;
            }

            board[ll][kk] = Some(copy_tile(lines, kk, ll, a));
        }
    }

    board
}

fn map_edges(board: &Board, k: usize, l: usize) -> EdgeMap {
    let mut map = EdgeMap::new();

    for ll in 0..l {
        let first = board[ll]
            .iter()
            .position(|maybe_tile| maybe_tile.is_some())
            .expect("Some tile at least once in row");

        let mut last = k - 1;
        for kk in (first + 1)..k {
            if board[ll][kk].is_some() {
                if map
                    .insert(((kk - 1, ll), Direction::Right), (kk, ll))
                    .is_some()
                {
                    panic!("Unexpected")
                }

                if map
                    .insert(((kk, ll), Direction::Left), (kk - 1, ll))
                    .is_some()
                {
                    panic!("Unexpected")
                }
            } else {
                last = kk - 1;
                break;
            }
        }

        if map
            .insert(((last, ll), Direction::Right), (first, ll))
            .is_some()
        {
            panic!("Unexpected")
        }

        if map
            .insert(((first, ll), Direction::Left), (last, ll))
            .is_some()
        {
            panic!("Unexpected")
        }
    }

    for kk in 0..k {
        let first = (0..l)
            .map(|ll| board[ll][kk].as_ref())
            .position(|maybe_tile| maybe_tile.is_some())
            .expect("Some tile at least once in column");

        let mut last = l - 1;

        for ll in (first + 1)..l {
            if board[ll][kk].is_some() {
                if map
                    .insert(((kk, ll - 1), Direction::Down), (kk, ll))
                    .is_some()
                {
                    panic!("Unexpected")
                }

                if map
                    .insert(((kk, ll), Direction::Up), (kk, ll - 1))
                    .is_some()
                {
                    panic!("Unexpected")
                }
            } else {
                last = ll - 1;
                break;
            }
        }

        if map
            .insert(((kk, last), Direction::Down), (kk, first))
            .is_some()
        {
            panic!("Unexpected")
        }

        if map
            .insert(((kk, first), Direction::Up), (kk, last))
            .is_some()
        {
            panic!("Unexpected")
        }
    }

    map
}

fn process_instructions(s: String) -> Vec<Instruction> {
    s.chars()
        .into_iter()
        .group_by(|&c| c.is_numeric())
        .into_iter()
        .map(|(are_numeric, mut group)| {
            if are_numeric {
                let num = group
                    .collect::<String>()
                    .parse::<usize>()
                    .expect("Number can be parsed to usize");

                Instruction::Move(num)
            } else {
                let c = group.next().expect("Single char is yielded");
                match c {
                    'L' => Instruction::TurnLeft,
                    'R' => Instruction::TurnRight,
                    _ => panic!("Unexpected character"),
                }
            }
        })
        .collect()
}

fn locate_start(
    board: &Board,
    k: usize,
    l: usize,
    a: usize,
) -> Option<(usize, usize, usize, usize)> {
    for ll in 0..l {
        for kk in 0..k {
            if let Some(tile) = &board[ll][kk] {
                for y in 0..a {
                    for x in 0..a {
                        if tile[y][x] == '.' {
                            return Some((kk, ll, x, y));
                        }
                    }
                }
            } else {
                continue;
            }
        }
    }

    None
}

fn main() {
    let board_lines = std::io::stdin()
        .lines()
        .map(|maybe_line| maybe_line.expect("A line is read"))
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut ins_str = String::new();
    std::io::stdin()
        .read_line(&mut ins_str)
        .expect("Instructions are read");

    // The maximum line length is also the maximum x coordinate
    let max_x_coord = board_lines
        .iter()
        .max_by_key(|line| line.len())
        .expect("Max is found")
        .len();
    // The max y coordinate is the length of the lines
    let max_y_coord = board_lines.len();

    // The tile size
    let a = gcd::binary_usize(max_x_coord, max_y_coord);
    let k = max_x_coord / a;
    let l = max_y_coord / a;

    let board = board_from_lines(&board_lines, k, l, a);

    let edge_map = map_edges(&board, k, l);
    let mut instructions = process_instructions(ins_str);
    instructions.reverse();

    let (mut pos_k, mut pos_l, mut pos_x, mut pos_y) =
        locate_start(&board, k, l, a).expect("Start can be found");
    let mut dir = Direction::Right;

    while let Some(instruction) = instructions.pop() {
        match instruction {
            Instruction::Move(mut n) => {
                let mut this_tile = board[pos_l][pos_k].as_ref().expect("Motion is consistent");

                match dir {
                    Direction::Up => {
                        while n > 0 {
                            if pos_y == 0 {
                                let &(new_k, new_l) = edge_map
                                    .get(&((pos_k, pos_l), dir))
                                    .expect("Edges are mapped");
                                let new_tile = board[new_l][new_k]
                                    .as_ref()
                                    .expect("Edges are mapped correctly");

                                if new_tile[a - 1][pos_x] == '#' {
                                    break;
                                } else {
                                    this_tile = new_tile;
                                    pos_k = new_k;
                                    pos_l = new_l;
                                    pos_y = a - 1;
                                }
                            } else if this_tile[pos_y - 1][pos_x] == '#' {
                                break;
                            } else {
                                pos_y -= 1;
                            }

                            n -= 1;
                        }
                    }
                    Direction::Down => {
                        while n > 0 {
                            if pos_y == a - 1 {
                                let &(new_k, new_l) = edge_map
                                    .get(&((pos_k, pos_l), dir))
                                    .expect("Edges are mapped");
                                let new_tile = board[new_l][new_k]
                                    .as_ref()
                                    .expect("Edges are mapped correctly");

                                if new_tile[0][pos_x] == '#' {
                                    break;
                                } else {
                                    this_tile = new_tile;
                                    pos_k = new_k;
                                    pos_l = new_l;
                                    pos_y = 0;
                                }
                            } else if this_tile[pos_y + 1][pos_x] == '#' {
                                break;
                            } else {
                                pos_y += 1;
                            }

                            n -= 1;
                        }
                    }
                    Direction::Left => {
                        while n > 0 {
                            if pos_x == 0 {
                                let &(new_k, new_l) = edge_map
                                    .get(&((pos_k, pos_l), dir))
                                    .expect("Edges are mapped");
                                let new_tile = board[new_l][new_k]
                                    .as_ref()
                                    .expect("Edges are mapped correctly");

                                if new_tile[pos_y][a - 1] == '#' {
                                    break;
                                } else {
                                    this_tile = new_tile;
                                    pos_k = new_k;
                                    pos_l = new_l;
                                    pos_x = a - 1;
                                }
                            } else if this_tile[pos_y][pos_x - 1] == '#' {
                                break;
                            } else {
                                pos_x -= 1;
                            }

                            n -= 1;
                        }
                    }
                    Direction::Right => {
                        while n > 0 {
                            if pos_x == a - 1 {
                                let &(new_k, new_l) = edge_map
                                    .get(&((pos_k, pos_l), dir))
                                    .expect("Edges are mapped");
                                let new_tile = board[new_l][new_k]
                                    .as_ref()
                                    .expect("Edges are mapped correctly");

                                if new_tile[pos_y][0] == '#' {
                                    break;
                                } else {
                                    this_tile = new_tile;
                                    pos_k = new_k;
                                    pos_l = new_l;
                                    pos_x = 0;
                                }
                            } else if this_tile[pos_y][pos_x + 1] == '#' {
                                break;
                            } else {
                                pos_x += 1;
                            }

                            n -= 1;
                        }
                    }
                }
            }
            Instruction::TurnLeft => dir = dir.rotate_ccw(),
            Instruction::TurnRight => dir = dir.rotate_cw(),
        }
    }

    let result = 1000 * (pos_l * a + pos_y + 1) + 4 * (pos_k * a + pos_x + 1) + dir.eval();

    println!("{result}");
}
