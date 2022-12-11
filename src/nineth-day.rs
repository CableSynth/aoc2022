use std::collections::{HashMap, HashSet};

use itertools::Itertools;
const TEST_STR: &str = include_str!("../test_data/test.txt");
const PROMPT_STR: &str = include_str!("../test_data/nine_test.txt");

fn parse_input(input: Vec<&str>) -> Vec<((i32, i32), u32)> {
    input
        .iter()
        .map(|l| {
            let (dir, dist) = l.trim().split_once(" ").unwrap();
            let d = match dir.trim() {
                "R" => (0, 1),
                "L" => (0, -1),
                "U" => (1, 0),
                "D" => (-1, 0),
                _ => (0, 0),
            };
            let dist: u32 = dist.parse().unwrap();
            (d, dist)
        })
        .collect_vec()
}

fn main() {
    let move_intstructions = PROMPT_STR.lines().collect_vec(); // need to parse input kek
    let moves = parse_input(move_intstructions);

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let (mut head, mut tail) = ((0, 0i32), (0, 0));
    visited.insert(tail);
    for m in moves.clone() {
        for _ in 0..m.1 {
            let new_head: (i32, i32) = (head.0 + m.0 .0, head.1 + m.0 .1);
            let dist = (new_head.0 - tail.0, new_head.1 - tail.1);
            if dist != (0, 0) {
                let dist_sqr = dist.0.pow(2) + dist.1.pow(2);
                if dist_sqr == 5 || dist_sqr == 4 {
                    tail = (tail.0 + dist.0.clamp(-1, 1), tail.1 + dist.1.clamp(-1, 1));
                    visited.insert(tail);
                    // } else if dist_sqr == 4{
                    // tail = (tail.0 + dist.0, tail.1- dist.1);
                    //     visited.insert(tail);
                }
            }

            head = new_head;
        }
    }

    println!("{}", visited.len());

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut knot_vec = vec![(0, 0); 10]; // indx 1-9 act as their own head
    visited.insert(knot_vec[9]);
    for m in moves {
        for _ in 0..m.1 {
            let mut new_head: (i32, i32) = (knot_vec[0].0 + m.0 .0, knot_vec[0].1 + m.0 .1);
            for i in 1..knot_vec.len() {
                let dist = (new_head.0 - knot_vec[i].0, new_head.1 - knot_vec[i].1);
                if dist != (0, 0) {
                    let dist_sqr = dist.0.pow(2) + dist.1.pow(2);
                    if dist_sqr >= 4 {
                        knot_vec[i] = (
                            knot_vec[i].0 + dist.0.clamp(-1, 1),
                            knot_vec[i].1 + dist.1.clamp(-1, 1),
                        );
                        visited.insert(knot_vec[9]);
                    }
                }

                knot_vec[i - 1] = new_head;
                new_head = knot_vec[i]
            }
            // println!("{:?}", knot_vec);
        }
    }

    println!("{}", visited.len());
}
