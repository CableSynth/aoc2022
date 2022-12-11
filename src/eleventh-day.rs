use itertools::Itertools;
use num::FromPrimitive;
use rug::{Assign, Integer};

const TEST_STR: &str = include_str!("../test_data/test.txt");
const PROMPT: &str = include_str!("../test_data/eleventh_day.txt");

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u128>,
    operation: Vec<String>,
    test: u128,
    destination: [u16; 2],
    inspections: u32,
}
#[derive(Debug, Clone)]
struct BigMonkey {
    items: Vec<usize>,
    operation: Vec<String>,
    test: usize,
    destination: [u16; 2],
    inspections: u32,
}

fn parse_data(input: &str) -> Monkey {
    let mut l = input.trim().lines().collect_vec();
    l.reverse();
    l.pop().unwrap();
    let items = l
        .pop()
        .unwrap()
        .trim()
        .split_once(":")
        .unwrap()
        .1
        .split(",")
        .map(|i| i.trim().parse::<u128>().unwrap())
        .collect_vec();
    let operation = l
        .pop()
        .unwrap()
        .trim()
        .split_once(":")
        .unwrap()
        .1
        .split_once("=")
        .unwrap()
        .1
        .split_whitespace()
        .map(|i| i.trim().to_string())
        .collect_vec();
    let test = l
        .pop()
        .unwrap()
        .trim()
        .split_whitespace()
        .last()
        .unwrap()
        .trim()
        .parse::<u128>()
        .unwrap();
    let if_true = l
        .pop()
        .unwrap()
        .trim()
        .split_whitespace()
        .last()
        .unwrap()
        .trim()
        .parse::<u16>()
        .unwrap();
    let if_false = l
        .pop()
        .unwrap()
        .trim()
        .split_whitespace()
        .last()
        .unwrap()
        .trim()
        .parse::<u16>()
        .unwrap();
    let destination = [if_true, if_false];
    Monkey {
        items,
        operation,
        test,
        destination,
        inspections: 0,
    }
}

fn part1(monkes: Vec<Monkey>) -> Vec<Monkey> {
    let mut monkeys = monkes.clone();
        for m in 0..monkeys.len() {
            monkeys[m].items.reverse();
            while let Some(item) = monkeys[m].items.pop()  {
                let monk = monkeys[m].clone(); 
                let mut temp = item.clone();
                let rhs = match monk.operation[2].parse::<u128>() {
                    Ok(i) => i,
                    Err(_) => temp,
                };
                if monk.operation[1] == "*" {
                    temp *= rhs;
                } else if monk.operation[1] == "+" {
                    temp += rhs;
                }
                temp = temp / 3;
                if temp % monk.test == 0 {
                    monkeys[monk.destination[0] as usize].items.push(temp);

                } else {
                    monkeys[monk.destination[1] as usize].items.push(temp);
                }
                monkeys[m].inspections += 1;
                
            }
    }
    // println!("{:?}", monkeys);
    monkeys
}

fn part2(monkes: Vec<BigMonkey>) -> Vec<BigMonkey> {
    let mut monkeys = monkes.clone();
    for m in 0..monkeys.len() {
        monkeys[m].items.reverse();
        while let Some(item) = monkeys[m].items.pop() {
            let monk = monkeys[m];
            let mut temp = item;
            let rhs = match monk.operation[2].parse::<Integer>() {
                Ok(i) => i,
                Err(_) => temp.clone(),
            };
            if monk.operation[1] == "*" {
                temp *= rhs;
            } else if monk.operation[1] == "+" {
                temp += rhs;
            }
            if temp.clone() % monk.test == 0 {
                monkeys[monk.destination[0] as usize].items.push(temp);

            } else {
                monkeys[monk.destination[1] as usize].items.push(temp);
            }
            monkeys[m].inspections += 1;
        }
    }
    monkeys
}

fn main() {
    // let instructions = parse_data(TEST_STR);
    let monkes = PROMPT.trim().split("\n\n").map(parse_data).collect_vec();
    let mut monkeys = monkes.clone();
    for _ in 0..20 {
        monkeys = part1(monkeys.clone());
    }
    for m in monkeys.iter() {
        println!("{:?}", m);
    }
    let mut max = monkeys.iter().sorted_by(|m1, m2| Ord::cmp(&m1.inspections, &m2.inspections)).collect_vec();
    max.reverse();
    let ans = max[0].inspections * max[1].inspections;
    println!("{}", ans);
    
    let mut monkeys = monkes.iter().map(|m| BigMonkey {
        items: m.items.iter().map(|i| Integer::from(*i)).collect_vec(),
        operation: m.operation.clone(),
        test: Integer::from(m.test),
        destination: m.destination,
        inspections: m.inspections,
    }).collect_vec();
    for i in 0..10000 {
        // println!("iteration: {}", i);
        monkeys = part2(monkeys.clone());
    }
    for m in monkeys.iter() {
        println!("{:?}", m);
    }
    let mut max = monkeys.iter().sorted_by(|m1, m2| Ord::cmp(&m1.inspections, &m2.inspections)).collect_vec();
    max.reverse();
    let ans = max[0].inspections * max[1].inspections;
    println!("{}", ans);

}
