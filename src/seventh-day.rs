use std::collections::HashMap;

const PROMPT_STR: &str = include_str!("../test_data/seventh_day.txt");

#[derive(Debug)]
struct Tree {
    size: u64,
    children: HashMap<String, Tree>,
}

impl Tree {
    fn new(size: u64) -> Tree {
        Tree {
            size: size,
            children: HashMap::new(),
        }
    }

    fn get_size(&self) -> u64 {
        self.size
            + self
                .children
                .values()
                .map(|tree| tree.get_size())
                .sum::<u64>()
    }

    fn traverse_and_insert(&mut self, path: &[String], key: String, val: u64) {
        if path.is_empty() {
            self.children.insert(key, Tree::new(val));
        } else {
            self.children
                .get_mut(path.first().unwrap())
                .unwrap()
                .traverse_and_insert(&path[1..], key, val);
        }
    }

    fn get_folder_sizes(&self) -> Vec<u64> {
        let mut out: Vec<u64> = self
            .children
            .values()
            .filter(|x| x.size == 0)
            .map(|x| x.get_folder_sizes())
            .flatten()
            .collect();
        out.push(self.get_size());
        out
    }
}

fn parse_input(filename: &str) -> Tree {
    // Parse input file into a tree structure. Return the tree root.
    let lines: Vec<_> = filename.lines().map(|x| x.to_string()).collect();

    let mut root: Tree = Tree::new(0);
    let mut path: Vec<String> = Vec::new();
    for line in lines.iter().skip(1) {
        match line.rsplit_once(' ') {
            Some(("$ cd", "..")) => {
                path.pop();
            }
            Some(("$ cd", name)) => {
                path.push(name.to_string());
            }
            Some(("$", "ls")) => (),
            Some(("dir", name)) => {
                root.traverse_and_insert(&path, name.to_string(), 0);
            }
            Some((size, name)) => {
                root.traverse_and_insert(&path, name.to_string(), size.parse().unwrap());
            }
            _ => (),
        }
    }
    root
}

fn solve() -> (u64, u64) {
    let root = parse_input(PROMPT_STR);
    let folder_sizes = root.get_folder_sizes();

    let part1 = folder_sizes.iter().filter(|&&x| x <= 100000).sum();

    let min_folder_size = 30000000 - (70000000 - root.get_size());
    let part2 = *folder_sizes
        .iter()
        .filter(|&&x| x >= min_folder_size)
        .min()
        .unwrap();

    (part1, part2)
}

fn main() {
    println!("{:?}", solve())
}
