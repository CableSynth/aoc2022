use itertools::Itertools;
const TEST_STR: &str = include_str!("../test_data/test.txt");
const PROMPT: &str = include_str!("../test_data/tenth_day.txt");

fn parse_data(input: &str) -> Vec<Option<i32>> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .split_once(" ")
                .and_then(|sp| sp.1.parse::<i32>().ok())
        })
        .collect_vec()
}

fn part1(instructions: Vec<Option<i32>>) -> (i32, String) {
    let mut x = 1;
    let mut cycle = 0;
    let mut s = 0;
    let mut screen_line: Vec<&str> = Vec::new();
    for instruction in instructions {
        let mut op = 0;
        let row = cycle / 40;
        let sprite_window = [x - 1, x, x + 1];
        // println!("sprite_window: {:?}", sprite_window);
        if cycle_check(cycle + 1) {
            s += (cycle + 1) * x;
        }
        if instruction.is_some() {
            if sprite_window.contains(&((cycle) % 40)) {
                screen_line.push("#");
            } else {
                screen_line.push(".");
            }
            cycle += 1;
            op = instruction.unwrap();
            if cycle_check(cycle + 1) {
                s += (cycle + 1) * x;
            }
        }
        if sprite_window.contains(&((cycle) % 40)) {
            screen_line.push("#");
        } else {
            screen_line.push(".");
        }
        cycle += 1;

        x += op;
    }
    return (s, screen_line.join(""));

    // instructions.iter().map(|i|  {
    //     let mut op = 0;
    //     if cycle_check(cycle) {
    //         return cycle*x
    //     }
    //     if i.is_some() {
    //         cycle += 1;
    //         op = i.unwrap();
    //         if cycle_check(cycle) {
    //             return cycle*x
    //         }
    //     }

    //     cycle += 1;
    //     x += op;
    //     0
    // }).sum::<i32>()
}

fn cycle_check(cycle: i32) -> bool {
    if cycle % 20 == 0 {
        if cycle == 20 {
            return true;
        }
        if (cycle - 20) % 40 == 0 {
            return true;
        }
        return false;
    }
    false
}

fn main() {
    let instructs = parse_data(PROMPT);
    let (res, screen_line) = part1(instructs);
    println!("res: {}", res);
    for line in &screen_line.chars().chunks(40) {
        println!("{}", line.collect::<String>());
    }
}
