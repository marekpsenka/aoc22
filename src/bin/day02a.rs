fn get_score(op_choice: char, my_choice: char) -> i32
{
    match op_choice {
        'A' => {
            match my_choice {
                'X' => 3 + 1,
                'Y' => 6 + 2,
                'Z' => 0 + 3,
                _ => panic!("My choice is invalid")

            }
        }
        'B' => {
            match my_choice {
                'X' => 0 + 1,
                'Y' => 3 + 2,
                'Z' => 6 + 3,
                _ => panic!("My choice is invalid")
            }

        }
        'C' => {
            match my_choice {
                'X' => 6 + 1,
                'Y' => 0 + 2,
                'Z' => 3 + 3,
                _ => panic!("My choice is invalid")

            }
        }
        _ => panic!("Invalid opponent choice")
    }
} 

fn main() {
    let mut my_score = 0;

    for line in std::io::stdin().lines() {
        let cs = line.unwrap().trim().chars().collect::<Vec<char>>();
        my_score += get_score(cs[0], cs[2]);
    }

    println!("{my_score}");
}
