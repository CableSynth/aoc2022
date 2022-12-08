use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const PROMPT_STR: &str = include_str!("../test_data/seventh_day.txt");
const TEST_STR: &str = include_str!("../test_data/test.txt");

struct TreeNode<'a> {
    pub name: &'a str,
    pub size: Option<u32>,
    pub children: Option<HashMap<&'a str, Rc<RefCell<TreeNode<'a>>>>>,
    pub parent: Option<Rc<RefCell<TreeNode<'a>>>>,
}

impl<'a> TreeNode<'a> {
    pub fn new() -> TreeNode<'a> {
        TreeNode {
            name: "/",
            size: None,
            children: None,
            parent: None,
        }
    }

    pub fn cd(self, location: &'a str) -> Rc<RefCell<TreeNode>>{
        match location {
            "/" => {
                if self.parent.is_none() {
                    return Rc::new(self.into())
                } else {
                    self.cd(location)
                }
            },
            ".." => {
                if self.parent.is_none() {
                    return Rc::new(self.into())
                } else {
                    return self.parent.unwrap()
                }
            },
            _ => {
               return self.children.unwrap().get(location).unwrap().clone()
            }
        }
    }

    pub fn add_child(&'a mut self, k: &'a str, v: TreeNode<'a>){
        self.children.as_mut().unwrap().insert(k, Rc::new(RefCell::new(v))).unwrap();
    }
}

fn parse_file_cmd(s: Vec<&str>) -> Option<FileLine> {
    match s[0] {
        "cd" => Some(FileLine {
            file_cmd: FileCommand::ChangeDirectory,
            file_name: s[1],
            value: None,
        }),
        "ls" => Some(FileLine {
            file_cmd: FileCommand::ListDirectory,
            file_name: ".",
            value: None,
        }),
        "dir" => Some(FileLine {
            file_cmd: FileCommand::DirectoryName,
            file_name: s[1],
            value: None,
        }),
        _ => Some(FileLine {
            file_cmd: FileCommand::FileName,
            file_name: s[1],
            value: s[0].to_string().parse::<u32>().ok(),
        }),
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum FileCommand {
    ChangeDirectory,
    ListDirectory,
    FileName,
    DirectoryName,
}

#[derive(Debug)]
struct FileLine<'a> {
    file_cmd: FileCommand,
    file_name: &'a str,
    value: Option<u32>,
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
    let mut tree = TreeNode::<'_>::new();
    let cwd = *TreeNode::<'_>::cd(tree, "/");
    for l in s {
        let cmd = l.file_cmd;
        match cmd {
            FileCommand::ChangeDirectory => {
                let location = l.file_name;
                cwd = *TreeNode::<'_>::cd(cwd, location);

            }
            FileCommand::ListDirectory => (),
            FileCommand::DirectoryName | FileCommand::FileName => {
                let name = l.file_name;
                let new_node = TreeNode {
                    name,
                    size: l.value.clone(),
                    children: Some(HashMap::new()),
                    parent: Some(Rc::new(cwd)),
                };
                cwd = cwd.add_child(name, new_node).clone();
            }
        }
    }

    dbg!(cwd);
}
