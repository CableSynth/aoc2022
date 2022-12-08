use std::char::EscapeDebug;

use itertools::Itertools;
const PROMPT_STR: &str = include_str!("../test_data/eighth_test.txt");

fn main() {

    let lin: Vec<_> = PROMPT_STR.lines().collect();
    let grid: Vec<Vec<_>> = lin.iter().map(|f| f.trim().chars().map(|i| i.to_digit(10).unwrap()).collect()).collect();
    let mut view_cout = 0;
    let mut best_score = 0;
    for (i, j) in (0..grid.len()).cartesian_product(0..grid[0].len()) {
        let current_height = grid[i][j];
        let mut view = false;
        let mut score = 1;
        for (dx, dy) in [(0,1), (0,-1), (1,0), (-1, 0)].iter() {
            let row: Vec<&u32> = (1..).map(|t| {
                grid.get((i as i32 + t *dy) as usize).and_then(|r| r.get((j as i32 + t *dx) as usize))
            }).while_some().collect();

            view |= row.iter().all(|&&h| h < current_height);
            let mut s = 0;
            for h in row.iter() {
                if **h < current_height {
                    s+=1
                } else {
                    s+=1;
                    break;
                }
            }
            score *=s;

        }
        best_score = best_score.max(score);
        view_cout += view as u32;
    }

    println!("{}", view_cout);
    println!("{}", best_score);
}