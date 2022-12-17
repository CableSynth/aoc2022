use std::cmp::{max, Ordering};

use itertools::Itertools;
use serde_json::Value;

const INPUT: &str = include_str!("../test_data/day-13.txt");
const TEST_INPUT: &str = include_str!("../test_data/test.txt");

fn compare(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x.as_u64().unwrap().cmp(&y.as_u64().unwrap()),
        (Value::Array(a), Value::Array(b)) => {
            for i in 0..max(a.len(), b.len()) {
                match (a.get(i), b.get(i)) {
                    (_, None) => return Ordering::Greater,
                    (None, _) => return Ordering::Less,
                    (Some(x), Some(y)) => match compare(x, y) {
                        Ordering::Equal => {}
                        c => return c,
                    },
                }
            }
            Ordering::Equal
        }
        (Value::Array(_), Value::Number(_)) => compare(a, &Value::Array(vec![b.clone()])),
        (Value::Number(_), Value::Array(_)) => compare(&Value::Array(vec![a.clone()]), b),
        _ => unreachable!(),
    }
}

fn main() {
    let mut input = INPUT
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .collect_vec();

    let part_1 = input
        .iter()
        .tuples()
        .positions(|(a, b)| compare(a, b) != Ordering::Greater)
        .map(|i| i + 1)
        .sum::<usize>();
    let dividers = [
        serde_json::from_str::<Value>("[[2]]").unwrap(),
        serde_json::from_str::<Value>("[[6]]").unwrap(),
    ];
    input.extend(dividers.iter().cloned());
    input.sort_by(|a, b| compare(a, b));
    let part_2 = input
        .iter()
        .positions(|b| dividers.contains(b))
        .map(|i| i + 1)
        .product::<usize>();
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
