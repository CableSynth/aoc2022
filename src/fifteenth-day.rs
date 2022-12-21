use itertools::Itertools;

const TEST: &str = include_str!("../test_data/test.txt");

fn distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    let square_sum = (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();

    square_sum
}

fn main() {
    let data = TEST
        .lines()
        .map(|x| {
            let (sensor_str, beacon_str) = x.split_once(": ").unwrap();
            let (sen_loc_x, sen_loc_y) = sensor_str["Sensor at ".len()..].split_once(", ").unwrap();
            let (bea_loc_x, bea_loc_y) = beacon_str["closest beacon is at ".len()..]
                .split_once(", ")
                .unwrap();
            (
                (
                    sen_loc_x["x=".len()..].parse::<i32>().unwrap(),
                    sen_loc_y["y=".len()..].parse::<i32>().unwrap(),
                ),
                (
                    bea_loc_x["x=".len()..].parse::<i32>().unwrap(),
                    bea_loc_y["y=".len()..].parse::<i32>().unwrap(),
                ),
            )
        })
        .collect_vec();

    println!("{:?}", data);
    let distances = data
        .iter()
        .map(|(sensor, beacon)| distance(sensor.clone(), beacon.clone()))
        .collect_vec();
    println!("{:?}", distances);
}