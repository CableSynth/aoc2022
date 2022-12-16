use itertools::Itertools;

const TEST: &str = include_str!("../test_data/test.txt");

fn main() {
    let mut area = vec![vec![false; 1000]; 1000];
    

    for l in TEST.lines() {
        let coords = l.split (" -> ").map(|s| {
            let (x, y) = s.split_at(s.find(',').unwrap());
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        });
        for ((x1, y1), (x2, y2)) in coords.tuple_windows() {
            
        }
    }
}