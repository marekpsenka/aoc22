use std::collections::HashSet;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let cs: Vec<char> = line.chars().collect();
    for pos in 14..cs.len() {
        let set: HashSet<char> = HashSet::from_iter(cs[(pos - 14)..pos].iter().cloned());
        if set.len() == 14 {
            println!("{pos}");
            break;
        }
    }
}
