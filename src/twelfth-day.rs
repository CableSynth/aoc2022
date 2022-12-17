use std::collections::HashMap;
use std::collections::VecDeque;

use itertools::Itertools;

const PROMPT_STR: &str = include_str!("../test_data/twelfth_day.txt");
const TEST_DATA: &str = include_str!("../test_data/test.txt");

struct Grid {
    grid: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(grid: Vec<Vec<u32>>) -> Grid {
        let width = grid[0].len();
        let height = grid.len();
        Grid {
            grid,
            width,
            height,
        }
    }

    fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        if self.inbounds(x, y + 1) {
            if self.grid[y + 1][x] <= self.grid[y][x] + 1 {
                neighbors.push((x, y + 1));
            }
        }
        if y != 0 {
            if self.grid[y - 1][x] <= self.grid[y][x] + 1 {
                neighbors.push((x, y - 1));
            }
        }
        if self.inbounds(x + 1, y) {
            if self.grid[y][x + 1] <= self.grid[y][x] + 1 {
                neighbors.push((x + 1, y));
            }
        }
        if x != 0 {
            if self.grid[y][x - 1] <= self.grid[y][x] + 1 {
                neighbors.push((x - 1, y));
            }
        }
        // println!("{:?}", (y, x, self.grid[y][x]));
        // println!("neighbors {:?}", neighbors);
        neighbors
    }
}

fn a_star(
    grid: Grid,
    start: (usize, usize),
    end: (usize, usize),
) -> (
    HashMap<(usize, usize), Option<(usize, usize)>>,
    HashMap<(usize, usize), u32>,
) {
    let mut frontier: VecDeque<(i64, (usize, usize))> = VecDeque::new();
    frontier.push_back((0, start));
    let mut came_from: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    let mut cost_so_far: HashMap<(usize, usize), u32> = HashMap::new();
    came_from.insert(start, None);
    cost_so_far.insert(start, 0);

    while !frontier.is_empty() {
        let current = frontier.pop_front().unwrap();

        if current.1 == end {
            break;
        }
        for next in grid.neighbors(current.1 .0, current.1 .1) {
            let new_cost = cost_so_far[&current.1] + 1;
            if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                cost_so_far.insert(next, new_cost);
                let priority = new_cost as i64
                    + (next.0 as i64 - end.0 as i64).abs()
                    + (next.1 as i64 - end.1 as i64).abs();
                frontier.push_back((priority, next));
                came_from.insert(next, Some(current.1));
            }
        }
    }

    return (came_from, cost_so_far);
}

fn reconstruct_path(
    came_from: HashMap<(usize, usize), Option<(usize, usize)>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut current = end;
    let mut path: Vec<(usize, usize)> = Vec::new();
    while current != start {
        path.push(current);
        if came_from.contains_key(&current) {
            current = came_from[&current].unwrap();
        } else {
            return None;
        }
    }
    path.push(start);
    path.reverse();
    Some(path)
}

fn main() {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut starts: Vec<(usize, usize)> = Vec::new();
    let grid = PROMPT_STR
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = (x, y);
                        starts.push((x, y));
                        0
                    }
                    'E' => {
                        end = (x, y);
                        25
                    }
                    _ => {
                        if c == 'a' {
                            starts.push((x, y));
                        }
                        c as u32 - 97
                    }
                })
                .collect_vec()
        })
        .collect_vec();
    // println!("{:?}", grid);
    println!("start {:?}", starts);
    // println!("end {:?}", end);
    let (came_from, cost_so_far) = a_star(Grid::new(grid.clone()), start, end);
    // dbg!(came_from.clone());
    let path = reconstruct_path(came_from, start, end);
    if path.is_some() {
        println!("{:?}", path.unwrap().len() - 1);
    }
    let results = starts
        .iter()
        .filter_map(|start| {
            let (came_from, cost_so_far) = a_star(Grid::new(grid.clone()), *start, end);
            let path = reconstruct_path(came_from, *start, end);
            path
        })
        .collect_vec();
    let res_len = results.iter().map(|x| x.len()).min();
    println!("{:?}", res_len.unwrap() - 1);
}
