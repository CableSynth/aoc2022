use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const PROMPT_STR: &str = include_str!("../test_data/seventh_day.txt");
const TEST_STR: &str = include_str!("../test_data/test.txt");

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    val: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }
}

#[derive(Debug, Default)]
struct ArenaTree<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq,
{
    fn node(&mut self, val: T) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        idx
    }
    fn size(&self) -> usize {
        self.arena.len()
    }
    fn edges(&self) -> usize {
        self.arena
            .iter()
            .fold(0, |acc, node| acc + node.children.len())
    }

    fn depth(&self, idx: usize) -> usize {
        match self.arena[idx].parent {
            Some(id) => 1 + self.depth(id),
            None => 0,
        }
    }
    fn depth_to_target(&self, idx: usize, target: &T) -> Option<usize> {
        // are we here?  If so, Some(0)
        if target == &self.arena[idx].val {
            return Some(0);
        }

        // If not, try all children
        for p in &self.arena[idx].children {
            if let Some(x) = self.depth_to_target(*p, &target) {
                return Some(1 + x);
            }
        }
        // If it cant be found, return None
        None
    }
    fn distance_between(&mut self, from: T, target: T) -> usize {
        // If it's not in the tree, this will add a new unconnected node
        // the final function will still return None
        let start_node = self.node(from);
        let mut ret = 0;
        // Start traversal
        let mut trav = &self.arena[start_node];
        // Explore all children, then hop up one
        while let Some(inner) = trav.parent {
            if let Some(x) = self.depth_to_target(inner, &target) {
                ret += x;
                break;
            }
            trav = &self.arena[inner];
            ret += 1;
        }
        // don't go all the way to target, just orbit
        if ret > 0 {
            ret - 1
        } else {
            ret
        }
    }
}

#[derive(Debug, Clone)]
struct TreeNode<'a> {
    pub name: &'a str,
    pub size: Option<u32>,
    pub children: Option<HashMap<&'a str, TreeNode<'a>>>,
    pub parent: Option<Box<&'a TreeNode<'a>>>,
}

impl<'a> TreeNode<'a> {
    pub fn new() -> TreeNode<'static> {
        TreeNode {
            name: "/",
            size: None,
            children: None,
            parent: None,
        }
    }

    pub fn cd(tn: &'a TreeNode<'a>, s: &str) -> Option<&'a TreeNode<'a>> {
        if s == ".." {
            if tn.parent.is_some() {
                return Some(&tn.parent.to_owned().unwrap());
            } else {
                return Some(tn);
            }
        }
        if s == "/" {
            if tn.parent.is_none() {
                return Some(tn);
            } else {
                TreeNode::<'_>::cd(tn, s)
            }
        } else {
            Some(&tn.children.as_ref().unwrap().get(s).expect(s))
        }
    }

    pub fn add_child(&'a mut self, k: &'a str, v: TreeNode<'a>)  -> &'a mut TreeNode<'a>{
        self.children.as_mut().unwrap().insert(k, v).unwrap();
        self
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
    let cwd = *TreeNode::<'_>::cd(&mut tree, "/").unwrap();
    for l in s {
        let cmd = l.file_cmd;
        match cmd {
            FileCommand::ChangeDirectory => {
                let location = l.file_name;
                cwd = *TreeNode::<'_>::cd(&cwd, location).expect(&location.clone());

            }
            FileCommand::ListDirectory => (),
            FileCommand::DirectoryName | FileCommand::FileName => {
                let name = l.file_name;
                let new_node = TreeNode {
                    name,
                    size: l.value.clone(),
                    children: Some(HashMap::new()),
                    parent: Some(Box::new(&cwd)),
                };
                cwd = cwd.add_child(name, new_node).clone();
            }
        }
    }

    dbg!(cwd);
}
