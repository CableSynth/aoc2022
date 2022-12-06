const PROMPT_STR: &str = include_str!("../test_data/fifth_test.txt");
const TEST_STR: &str = include_str!("../test_data/test.txt");

struct DynamicZip<I>
where
    I: Iterator,
{
    iterators: Vec<I>,
}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn transpose<T>(mut v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    for inner in &mut v {
        inner.reverse();
    }
    (0..v[0].len())
        .map(|_| {
            v.iter_mut()
                .filter_map(|inner| inner.pop())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn parse_stack(stack: &str) -> Vec<Vec<char>> {
    let mut l: Vec<&str> = stack.lines().collect();
    let l_end: Vec<_> = l.pop().unwrap().char_indices().collect();
    let points: Vec<_> = l_end
        .into_iter()
        .filter_map(|f| f.1.is_digit(10).then(|| f))
        .collect();
    let mut better_box: Vec<Vec<char>> = Vec::new();
    for _ in 0..points.clone().len() as usize {
        better_box.push(Vec::new())
    }
    for b in l.into_iter().rev() {
        split_row(b, &mut better_box, &points)
    }


    let better_box = better_box
        .into_iter()
        .map(|f| {
            f.into_iter()
                .filter_map(|c| (!c.is_whitespace()).then(|| c))
                .collect::<Vec<_>>()
        })
        .collect();
    return better_box;
}

fn split_row(l: &str, stack: &mut Vec<Vec<char>>, points: &Vec<(usize, char)>) {
    let r: Vec<_> = l.chars().collect();
    for (lin, s) in points.iter().zip(stack.iter_mut()) {
        s.push(r[lin.0])
    }
}

fn parse_moves(instructions: &str) -> Vec<Vec<usize>> {
    let instruction_lines: Vec<Vec<usize>> = instructions.lines().map(parse_move_line).collect();
    instruction_lines
}

fn parse_move_line(ml: &str) -> Vec<usize> {
    let instruct: Vec<_> = ml
        .trim()
        .split_whitespace()
        .filter_map(|f| f.parse::<usize>().ok())
        .collect();
    instruct
}

fn process_moves(moves: &Vec<Vec<usize>>, stack: &Vec<Vec<char>>) -> Vec<char> {
    let mut s = stack.clone();
    for instruction_line in moves {
        let mv_amount = instruction_line[0];
        let start = instruction_line[1] - 1;
        let end = instruction_line[2] - 1;
        for i in 0..mv_amount {
            let crate_to_move = s[start].pop().unwrap();
            s[end].push(crate_to_move);
        }
    }

    let tops: Vec<_> = s.iter().filter_map(|v| v.last()).collect();
    let t: Vec<_> = tops.into_iter().map(|f| f.to_owned()).collect();
    t
}

fn process_multi_moves(moves: &Vec<Vec<usize>>, stack: &Vec<Vec<char>>) -> Vec<char> {
    let mut s = stack.clone();
    for instruction_line in moves {
        let mv_amount = instruction_line[0];
        let start = instruction_line[1] - 1;
        let end = instruction_line[2] - 1;
        if mv_amount > 1 {
            let temp_s = s.clone();
            let crates_to_move = temp_s[start].split_at(temp_s[start].len() - mv_amount);
            s[start] = crates_to_move.0.to_vec();
            s[end].append(&mut crates_to_move.1.to_vec());
        } else {
            let crate_to_move = s[start].pop().unwrap();
            s[end].push(crate_to_move);
        }
    }

    let tops: Vec<_> = s.iter().filter_map(|v| v.last()).collect();
    let t: Vec<_> = tops.into_iter().map(|f| f.to_owned()).collect();
    t
}

fn main() {
    let (stack, instructs) = PROMPT_STR.split_once("\n\n").unwrap();
    let stack = parse_stack(stack);
    let instructs = parse_moves(instructs);
    let tops = process_moves(&instructs, &stack);
    println!("{:?}", tops);
    let tops = process_multi_moves(&instructs, &stack);
    println!("{:?}", tops);
}
