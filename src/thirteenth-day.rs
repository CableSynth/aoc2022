use itertools::Itertools;

const INPUT: &str = include_str!("../test_data/day-13.txt");
const TEST_INPUT: &str = include_str!("../test_data/test.txt");

fn parse_pair(pair:&str) -> bool {
    let left_right = pair.lines().map(|line| line.chars().collect_vec()).collect_vec();
    let mut left = left_right[0].clone();
    println!("{:?}", left);
    let mut right = left_right[1].clone();
    println!("{:?}", right);
    let mut parantheses = 0;
    for (l, r) in left.iter().zip(left.iter())) {
        if  {
            return true;
        }
    }
    true
}

fn main() {
    let input = TEST_INPUT.split("\n\n").collect_vec();
    let result = input.iter().map(|pair| parse_pair(pair)).collect_vec();
}