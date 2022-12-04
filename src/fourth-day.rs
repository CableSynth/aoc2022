use std::ops::RangeBounds;

const TEST_STR: &str = include_str!("../test_data/fourth_day.txt");
fn read() -> u32 {
    TEST_STR.trim().lines().map(parse_line).sum::<u32>()
}
fn read2() -> u32 {
    TEST_STR.trim().lines().map(parse_line_1).sum::<u32>()
}
fn parse_line(l: &str) -> u32 {
    let (first_section, second_section) = l.trim().split_once(",").unwrap();
    let first_section: Vec<u32> = first_section
        .split("-")
        .map(|f| f.parse::<u32>().unwrap())
        .collect();
    let second_section: Vec<u32> = second_section
        .split("-")
        .map(|f| f.parse::<u32>().unwrap())
        .collect();
    let first_section = std::ops::RangeInclusive::new(
        first_section.first().unwrap(),
        first_section.last().unwrap(),
    );
    let second_section = std::ops::RangeInclusive::new(
        second_section.first().unwrap(),
        second_section.last().unwrap(),
    );
    if (first_section.contains(&second_section.start())
        && first_section.contains(&second_section.end()))
        || (second_section.contains(&first_section.start())
            && second_section.contains(&first_section.end()))
    {
        return 1;
    }
    0
}

fn parse_line_1(l: &str) -> u32 {
    let (first_section, second_section) = l.trim().split_once(",").unwrap();
    let first_section: Vec<u32> = first_section
        .split("-")
        .map(|f| f.parse::<u32>().unwrap())
        .collect();
    let second_section: Vec<u32> = second_section
        .split("-")
        .map(|f| f.parse::<u32>().unwrap())
        .collect();
    if first_section.first() <= second_section.last()
        && second_section.first() <= first_section.last()
    {
        return 1;
    }
    0
}

fn main() {
    let sum_sections = read();
    let ovelap = read2();

    println!("{}", sum_sections);
    println!("{}", ovelap);
}
