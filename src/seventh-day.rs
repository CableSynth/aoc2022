use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const PROMPT_STR: &str = include_str!("../test_data/seventh_day.txt");
const TEST_STR: &str = include_str!("../test_data/test.txt");

struct TreeNode {
    pub name: String,
    pub size: Option<u32>,
    pub children: Option<HashMap<String, Rc<RefCell<TreeNode>>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

struct Tree {}

impl TreeNode {
    pub fn new() -> TreeNode {
        TreeNode {
            name: "".to_string(),
            size: None,
            children: None,
            parent: None,
        }
    }
}

fn main() {
    let s: Vec<_> = TEST_STR.trim().lines().collect();
    println!("{:?}", s);
}
