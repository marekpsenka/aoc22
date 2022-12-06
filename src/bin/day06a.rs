use std::collections::HashSet;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let cs: Vec<char> = line.chars().collect();
    for pos in 4..cs.len() {
        let set: HashSet<char> = HashSet::from_iter(cs[(pos - 4)..pos].iter().cloned());
        if set.len() == 4 {
            println!("{pos}");
            break;
        }
    }
}
