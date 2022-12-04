const TEST_STR: &str = include_str!("../test_data/first_test_1.txt");
fn read() -> Vec<u32> {
    TEST_STR
        .trim()
        .split("\n\n")
        .map(parse_elf_calories)
        .collect()
}

fn parse_elf_calories(value: &str) -> u32 {
    value.lines().flat_map(|l| l.parse::<u32>()).sum::<u32>()
}

fn main() {
    let cal_sum = read();

    let first_max = cal_sum.iter().max().unwrap();
    println!("{}", first_max);

    let mut top_3 = [u32::MIN; 3];
    for cal in cal_sum.iter() {
        let mut cal = *cal;
        for top_val in top_3.iter_mut() {
            if cal > *top_val {
                std::mem::swap(top_val, &mut cal);
            }
        }
    }

    println!("{:?}", top_3.iter().sum::<u32>());
}
