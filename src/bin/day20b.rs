fn untangle(shuffled: &[usize], numbers: &Vec<i64>) -> Vec<i64> {
    let mut untangled = vec![0; numbers.len()];
    numbers
        .iter()
        .enumerate()
        .for_each(|(i, &n)| untangled[shuffled[i]] = n);

    untangled
}

fn mix(numbers: &Vec<i64>, v: &mut Vec<usize>) {
    for i in 0..v.len() {
        let value = numbers[i];
        let pos = v[i] as i64;

        let newpos = (pos + value).rem_euclid(v.len() as i64 - 1);

        match pos.cmp(&newpos) {
            std::cmp::Ordering::Less => {
                for j in 0..v.len() {
                    if v[j] > pos as usize && v[j] <= newpos as usize {
                        v[j] -= 1
                    }
                }
            }
            std::cmp::Ordering::Greater => {
                for j in 0..v.len() {
                    if v[j] < pos as usize && v[j] >= newpos as usize {
                        v[j] += 1
                    }
                }
            }
            std::cmp::Ordering::Equal => {

            },
        }

        v[i] = newpos as usize;
    }
}

fn main() {
    let numbers = std::io::stdin()
        .lines()
        .map(|maybe_line| {
            let line = maybe_line.expect("A line is read");
            let num = line.parse::<i64>().expect("Successful parse");
            num * 811_589_153
        })
        .collect::<Vec<i64>>();

    let l = numbers.len();

    let mut mixed = (0..l).into_iter().collect::<Vec<usize>>();

    for _ in 0..10 {
        mix(&numbers, &mut mixed);
    }

    let untangled = untangle(&mixed, &numbers);
    let zero_pos = untangled
        .iter()
        .position(|&n| n == 0)
        .expect("Zero is found");

    let result = untangled[(zero_pos + 1000) % l] + untangled[(zero_pos + 2000) % l]
        + untangled[(zero_pos + 3000) % l];

    println!("{result}")
}
