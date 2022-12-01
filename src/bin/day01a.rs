use itertools::Itertools;

fn main() {
    let maybe_lines: Result<Vec<String>, _> = std::io::stdin().lines().collect();
    let lines = maybe_lines.unwrap();

    let max = lines.into_iter()
                   .group_by(|l| l.is_empty())
                   .into_iter()
                   .filter_map(
                   |(empty, g)| {
                       if empty {
                           None
                       }
                       else
                       {
                           let items: Result<Vec<i32>, _> = g.map(|l| l.trim().parse::<i32>())
                                                               .collect();
                           Some(items.unwrap().iter().sum::<i32>())
                       }
                   }).max().unwrap();

    println!("{max}");
}
