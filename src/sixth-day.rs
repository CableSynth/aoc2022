const PROMPT_STR: &str = include_str!("../test_data/sixth_test.txt");
const TEST_STR: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
const TEST_STR_1: &str = "nppdvjthqldpwncqszvftbrmjlhg";

fn main() {
    let s: Vec<_> = PROMPT_STR.trim().chars().collect();
    let mut start_of_packet = s[..4].to_vec();

    for (i, c) in s[4..].into_iter().enumerate() {
        let l = start_of_packet.len();
        let mut valid = false;
        for i in 0..l {
            let cmp = start_of_packet[i];

            let mut temp = start_of_packet.clone();
            temp.remove(i);
            valid = temp.iter().all(|f| f != &cmp);
            if !valid {
                break;
            }
        }
        if valid {
            println!("{}", i + 4);
            break;
        }
        start_of_packet.rotate_left(1);
        start_of_packet.pop();
        start_of_packet.push(c.to_owned());
    }
    let mut start_of_packet = s[..14].to_vec();

    for (i, c) in s[14..].into_iter().enumerate() {
        let l = start_of_packet.len();
        let mut valid = false;
        for i in 0..l {
            let cmp = start_of_packet[i];

            let mut temp = start_of_packet.clone();
            temp.remove(i);
            valid = temp.iter().all(|f| f != &cmp);
            if !valid {
                break;
            }
        }
        if valid {
            println!("{}", i + 14);
            break;
        }
        start_of_packet.rotate_left(1);
        start_of_packet.pop();
        start_of_packet.push(c.to_owned());
    }
}
