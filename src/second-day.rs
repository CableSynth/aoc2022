const TEST_STR: &str = include_str!("../test_data/second_test.txt");
fn read() -> (u32, u32) {
    (
        TEST_STR.trim().lines().map(parse_line).sum::<u32>(),
        TEST_STR.trim().lines().map(parse_game).sum::<u32>(),
    )
}

fn parse_line(l: &str) -> u32 {
    let split_vec: Vec<&str> = l.trim().splitn(2, ' ').collect();
    match split_vec[1] {
        "X" => {
            1 + match split_vec[0] {
                "A" => 3,
                "C" => 6,
                _ => 0,
            }
        }
        "Y" => {
            2 + match split_vec[0] {
                "A" => 6,
                "B" => 3,
                _ => 0,
            }
        }
        "Z" => {
            3 + match split_vec[0] {
                "B" => 6,
                "C" => 3,
                _ => 0,
            }
        }
        _ => 0,
    }
}

fn parse_game(l: &str) -> u32 {
    let split_vec: Vec<&str> = l.trim().splitn(2, ' ').collect();
    match split_vec[1] {
        "X" => match split_vec[0] {
            "A" => 3,
            "B" => 1,
            "C" => 2,
            _ => 0,
        },
        "Y" => {
            3 + match split_vec[0] {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => 0,
            }
        }
        "Z" => {
            6 + match split_vec[0] {
                "A" => 2,
                "B" => 3,
                "C" => 1,
                _ => 0,
            }
        }
        _ => 0,
    }
}

fn main() {
    let (cal_sum, super_duper) = read();

    println!("{}", cal_sum);
    println!("{}", super_duper);
}
