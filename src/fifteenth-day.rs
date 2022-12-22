use itertools::Itertools;

const TEST: &str = include_str!("../test_data/day-15.txt");

fn distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    let square_sum = (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();

    square_sum
}

fn solve_part1(beacons: &[(i64, i64, i64)]) -> i64 {
    let compressed = beacons
        .iter()
        .map(|&(sensor_x, sensor_y, distance)| (sensor_x, distance - (2000000 - sensor_y).abs()))
        .filter(|&(_, closeness)| closeness >= 0)
        .flat_map(|(x, closeness)| [(x - closeness, true), (x + closeness + 1, false)])
        .sorted()
        .collect::<Vec<_>>();
    let (mut ans, mut inside) = (-1, 1);
    for ((prev, _), &(x, start)) in compressed.iter().tuple_windows() {
        if inside > 0 {
            ans += x - prev
        }
        inside += if start { 1 } else { -1 };
    }
    ans
}
fn solve_part2(beacons: &[(i64, i64, i64)]) -> i64 {
    for &(x, y, distance) in beacons {
        for (dx, dy) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
            for dist in 0..distance {
                let hidden_beacon_x = x + dx * dist;
                let hidden_beacon_y = y + dy * (distance + 1 - dist);
                if hidden_beacon_x < 0 || hidden_beacon_y < 0 || hidden_beacon_x > 4000000 || hidden_beacon_y > 4000000 {
                    break;
                }
                if beacons.iter().all(|&(x, y, d)| (hidden_beacon_x - x).abs() + (hidden_beacon_y - y).abs() >= d) {
                    return hidden_beacon_x * 4000000 + hidden_beacon_y;
                }
            }
        }
    }
    unreachable!()
}
fn main() {
    let (mut max_x, mut min_x) = (0, 0);
    let beacons = TEST
        .lines()
        .map(|l| {
            l.split(|c: char| !c.is_digit(10) && c != '-')
                .filter_map(|w| w.parse::<i64>().ok())
                .collect_tuple()
                .map(|(x, y, beacon_x, beacon_y)| {
                    (x, y, (x - beacon_x).abs() + (y - beacon_y).abs())
                })
                .unwrap()
        })
        .collect::<Vec<_>>();
    let ans = solve_part1(beacons.as_slice());
    println!("Part 1: {}", ans);
    let ans = solve_part2(beacons.as_slice());
    println!("Part 2: {}", ans)
}
