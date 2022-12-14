use itertools::Itertools;

const INPUT: &str = include_str!("../test_data/day-13.txt");
const TEST_INPUT: &str = include_str!("../test_data/test.txt");


fn main() {
    let input = TEST_INPUT.split("\n\n").map(|x| x.split("\n").map(|line| line.split_terminator(&['[',']'][..]).filter_map(|s| {
        if s.is_empty() || s == "," {
            None
        } else {
            Some(s)
        }
    }).collect_vec()).collect_vec()).collect_vec();
    println!("{:?}", input);
}