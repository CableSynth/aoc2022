use itertools::Itertools;
use std::cmp::max;

const TEST: &str = include_str!("../test_data/test.txt");
const INPUT: &str = include_str!("../test_data/day-14.txt");

fn solve(mut area: Vec<Vec<bool>>, floor: usize, cliff: usize) -> usize {
    for grains in 0.. {
        let (mut x, mut y) = (500, 0);
        while y + 1 < floor {
            let Some(&dx) = [0,-1,1].iter().find(|&&dx| {
                !area[(x as i64 + dx) as usize][y+1]}) else { break };
            x = (x as i64 + dx) as usize;
            y += 1;
        }
        if y == cliff {
            return grains;
        }
        area[x][y] = true;
    }
    unreachable!()
}

fn main() {
    let mut area = vec![vec![false; 1000]; 1000];
    let mut max_y = 0;
    for l in INPUT.lines() {
        let coords = l.split(" -> ").map(|s| {
            let (x, y) = s.split_once(",").unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        });
        for ((x1, y1), (x2, y2)) in coords.tuple_windows() {
            max_y = max(max_y, max(y1, y2));

            let (mut x1, mut y1, x2, y2) = (x1 as isize, y1 as isize, x2 as isize, y2 as isize);
            let dx = (x2 - x1).signum();
            let dy = (y2 - y1).signum();
            area[x1 as usize][y1 as usize] = true;
            while (x1, y1) != (x2, y2) {
                x1 += dx;
                y1 += dy;
                area[x1 as usize][y1 as usize] = true;
            }
        }
    }
    println!("Part 1: {}", solve(area.clone(), max_y + 2, max_y + 1));
    println!("Part 2: {}", solve(area, max_y + 2, 0) + 1);
}
