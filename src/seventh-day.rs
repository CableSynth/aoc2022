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
            name: "/".to_string(),
            size: None,
            children: None,
            parent: None,
        }
    }

    pub fn from(s: &str) {

    }

    pub fn add_child() {

    }

}

fn parse_file_cmd(s: Vec<&str>) -> Option<FileLine>{
    match s[0] {
        "cmd" => Some(FileLine {
            file_cmd: FileCommand::ChangeDirectory,
            file_name: s[1].to_string(),
            value: None,
        }),
        "ls" => Some(FileLine {
            file_cmd: FileCommand::ListDirectory,
            file_name: ".".to_string(),
            value: None,
        }),
        "dir" => Some(FileLine { file_cmd: FileCommand::DirectoryName, file_name: s[1].to_string(), value: None }),
        _=> Some(FileLine { file_cmd: FileCommand::FileName, file_name: s[1].to_string(), value: Some(s[0].to_string()) }),
    }
}

#[derive(Debug)]
enum FileCommand {
    ChangeDirectory, 
    ListDirectory,
    FileName,
    DirectoryName,
}

#[derive(Debug)]
struct FileLine {
    file_cmd: FileCommand,
    file_name: String,
    value: Option<String>,
}

fn parse_line(s: &str) -> FileLine {
    let parts: Vec<_> = s.trim().split_whitespace().collect();
    println!("{:?}", parts);
    if parts.iter().next().unwrap().to_owned() == "$" {
        parse_file_cmd(parts[1..].to_vec()).unwrap()
    } else {
        parse_file_cmd(parts).unwrap()
    }
}

fn main() {
    let s: Vec<_> = TEST_STR.trim().lines().map(parse_line).collect();
    println!("{:?}", s);
    let mut tree = TreeNode::new();
    for l in s {

    }
}
