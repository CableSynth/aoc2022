use itertools::Itertools;

const TEST: &str = include_str!("../test_data/test.txt");


fn main() {
   let data = TEST.lines().map(|x| {
        let (sensor_str, beacon_str) = x.split_once(": ").unwrap();
        let (sen_loc_x, sen_loc_y) = sensor_str["Sensor at ".len()..].split_once(", ").unwrap();
        let (bea_loc_x, bea_loc_y) = beacon_str["closest beacon is at ".len()..].split_once(", ").unwrap();
        (
            (sen_loc_x["x=".len()..].parse::<i32>().unwrap(), sen_loc_y["y=".len()..].parse::<i32>().unwrap()),
            (bea_loc_x["x=".len()..].parse::<i32>().unwrap(), bea_loc_y["y=".len()..].parse::<i32>().unwrap())
        )
   }).collect_vec(); 

   println!("{:?}", data);
}