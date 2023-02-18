use itertools::Itertools;
use std::collections::HashMap;

type MonkeyMap = HashMap<String, Expr>;

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

fn main() {
    let m = MonkeyMap::from_iter(std::io::stdin().lines().map(|maybe_line| {
        let line = maybe_line.unwrap();
        parse_monkey(line)
    }));

    let result = evaluate("root", &m);
    println!("{result}")
}
