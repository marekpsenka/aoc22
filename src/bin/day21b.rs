use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Position {
    Left,
    Right
}

type MonkeyMap = HashMap<String, Expr>;
type InvMap = HashMap<String, (String, Position)>;

#[derive(Debug)]
enum Expr {
    Constant(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

fn parse_monkey(s: String) -> (String, Expr) {
    let (name, rhs) = s
        .split(": ")
        .collect_tuple()
        .expect("String can be split into two parts");

    let expr = if let Ok(num) = rhs.trim().parse::<i64>() {
        Expr::Constant(num)
    } else if rhs.contains('+') {
        let (left, right) = rhs
            .split(" + ")
            .collect_tuple()
            .expect("Expression can be split around +");

        Expr::Add(left.to_string(), right.to_string())
    } else if rhs.contains('-') {
        let (left, right) = rhs
            .split(" - ")
            .collect_tuple()
            .expect("Expression can be split around -");

        Expr::Sub(left.to_string(), right.to_string())
    } else if rhs.contains('*') {
        let (left, right) = rhs
            .split(" * ")
            .collect_tuple()
            .expect("Expression can be split around -");

        Expr::Mul(left.to_string(), right.to_string())
    } else if rhs.contains('/') {
        let (left, right) = rhs
            .split(" / ")
            .collect_tuple()
            .expect("Expression can be split around -");

        Expr::Div(left.to_string(), right.to_string())
    } else {
        panic!("Invalid rhs format");
    };

    (name.to_string(), expr)
}

fn evaluate(name: &str, m: &MonkeyMap) -> i64 {
    let expr = m.get(name).expect("Name has entry in map");
    match expr {
        Expr::Constant(n) => *n,
        Expr::Add(n1, n2) => evaluate(n1, m) + evaluate(n2, m),
        Expr::Sub(n1, n2) => evaluate(n1, m) - evaluate(n2, m),
        Expr::Mul(n1, n2) => evaluate(n1, m) * evaluate(n2, m),
        Expr::Div(n1, n2) => evaluate(n1, m) / evaluate(n2, m),
    }
}

fn get_operands(e: &Expr) -> Option<(String, String)> {
    match e {
        Expr::Constant(_) => None,
        Expr::Add(n1, n2) => Some((n1.clone(), n2.clone())),
        Expr::Sub(n1, n2) => Some((n1.clone(), n2.clone())),
        Expr::Mul(n1, n2) => Some((n1.clone(), n2.clone())),
        Expr::Div(n1, n2) => Some((n1.clone(), n2.clone())),
    }
}

fn build_inv_map(m: &MonkeyMap) -> InvMap {
    let mut inv = InvMap::new();
    for (name, expr) in m.iter() {
        if let Some((n1, n2)) = get_operands(expr) {
            inv.insert(n1, (name.clone(), Position::Left));
            inv.insert(n2, (name.clone(), Position::Right));
        }
    }
    inv
}

fn flip_expression(lhs: String, expr: Expr, pos: Position) -> Expr {
    match expr {
        Expr::Add(n1, n2) => {
            match pos {
                Position::Left => Expr::Sub(lhs, n2),
                Position::Right => Expr::Sub(lhs, n1),
            }
        },
        Expr::Sub(n1, n2) => {
            match pos {
                Position::Left => Expr::Add(lhs, n2),
                Position::Right => Expr::Sub(n1, lhs)
            }
        },
        Expr::Mul(n1, n2) => {
            match pos {
                Position::Left => Expr::Div(lhs, n2),
                Position::Right => Expr::Div(lhs, n1)
            }
        },
        Expr::Div(n1, n2) => {
            match pos {
                Position::Left => Expr::Mul(lhs, n2),
                Position::Right => Expr::Div(n1, lhs)
            }
        },
        Expr::Constant(_) => panic!("Cannot flip constant expression"),
    }
}

fn express_humn(m: &mut MonkeyMap, inv: &InvMap) {
    if m.remove("humn").is_none() {
        panic!("humn not originally present")
    }

    let mut next = "humn".to_string();
    loop {
        let (lhs, pos) = inv.get(&next).expect("next is in inverse map");
        let expr = m.remove(lhs).expect("entry to reorder can be found in monkey map");
        if lhs == "root" {
            let (left, right) = get_operands(&expr).expect("Expression has operands");
            match pos {
                Position::Left => {
                    m.insert(next, Expr::Constant(evaluate(&right, m)));
                },
                Position::Right => {
                    m.insert(next, Expr::Constant(evaluate(&left, m)));
                },
            }
            break;
        }

        let new_expr = flip_expression(lhs.clone(), expr, *pos);
        // println!("{:?} {:?}", next, new_expr);
        if m.insert(next, new_expr).is_some() {
            panic!("Unexpected")
        };
        next = lhs.clone();
    }
}

fn main() {
    let mut m = MonkeyMap::from_iter(std::io::stdin().lines().map(|maybe_line| {
        let line = maybe_line.unwrap();
        parse_monkey(line)
    }));

    let inv = build_inv_map(&m);
    express_humn(&mut m, &inv);
    let humn = evaluate("humn", &m);
    println!("{humn}")
}
