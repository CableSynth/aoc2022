const TEST_STR: &str = include_str!("../test_data/third_day.txt");
fn read() -> u32 {
    TEST_STR.trim().lines().map(parse_line).sum::<u32>()
}
fn read_2() -> u32 {
    let multi_line: Vec<&str> = TEST_STR.trim().lines().collect();
    parse_lines(multi_line)
}
fn parse_line(l: &str) -> u32 {
    let (first_half, second_half) = l.trim().split_at(l.len() / 2);
    let f: Vec<u32> = char_transform(first_half);
    let s: Vec<u32> = char_transform(second_half);
    let prio: Vec<u32> = f
        .iter()
        .filter_map(|i| (s.clone().into_iter().find(|x| x == i)))
        .collect();
    prio[0]
}

fn parse_lines(ls: Vec<&str>) -> u32 {
    let c: Vec<u32> = ls
        .chunks(3)
        .into_iter()
        .map(|l| {
            let (first, second, third) = (
                char_transform(l[0]),
                char_transform(l[1]),
                char_transform(l[2]),
            );
            let test: Vec<u32> = first
                .iter()
                .filter_map(|f| {
                    second
                        .clone()
                        .iter()
                        .find_map(|s| (third.clone().into_iter().find(|t| t == f && f == s)))
                })
                .collect();

            test[0]
        })
        .collect();
    c.iter().sum()
}

fn char_transform(s: &str) -> Vec<u32> {
    s.chars()
        .map(|c| {
            if c.is_lowercase() {
                c as u32 - 96
            } else {
                c as u32 - 38
            }
        })
        .collect()
}

fn main() {
    let prio_sum = read();
    let tri_sum = read_2();

    println!("{}", prio_sum);
    println!("{}", tri_sum);
}
