fn untangle(shuffled: &[usize], numbers: &Vec<i32>) -> Vec<i32> {
    let mut untangled = vec![0; numbers.len()];
    numbers
        .iter()
        .enumerate()
        .for_each(|(i, &n)| untangled[shuffled[i]] = n);

    untangled
}

fn main() {
    let numbers = std::io::stdin()
        .lines()
        .map(|maybe_line| {
            let line = maybe_line.expect("A line is read");
            line.parse::<i32>().expect("Successful parse")
        })
        .collect::<Vec<i32>>();

    let l = numbers.len();

    let mut shuffled = (0..l).into_iter().collect::<Vec<usize>>();

    for i in 0..l {
        let value = numbers[i];
        let pos = shuffled[i] as i32;

        let newpos = (pos + value).rem_euclid(l as i32 - 1);

        match pos.cmp(&newpos) {
            std::cmp::Ordering::Less => {
                for j in 0..l {
                    if shuffled[j] > pos as usize && shuffled[j] <= newpos as usize {
                        shuffled[j] -= 1
                    }
                }
            }
            std::cmp::Ordering::Greater => {
                for j in 0..l {
                    if shuffled[j] < pos as usize && shuffled[j] >= newpos as usize {
                        shuffled[j] += 1
                    }
                }
            }
            std::cmp::Ordering::Equal => {

            },
        }

        shuffled[i] = newpos as usize;
    }

    let untangled = untangle(&shuffled, &numbers);
    let zero_pos = untangled
        .iter()
        .position(|&n| n == 0)
        .expect("Zero is found");

    let result = untangled[(zero_pos + 1000) % l] + untangled[(zero_pos + 2000) % l]
        + untangled[(zero_pos + 3000) % l];

    println!("{result}")
}