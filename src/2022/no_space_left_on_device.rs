use std::{collections::HashMap, fs::File, io::Read};

#[derive(Debug)]
enum ItemType {
    File,
    Dir,
}
struct FileTree {
    root: Option<Box<Node>>,
}
impl FileTree {
    fn new() -> Self {
        FileTree {
            root: Some(Box::new(Node {
                item_type: ItemType::Dir,
                size: 0,
                children: None,
                parent: None,
            })),
        }
    }
}
#[derive(Debug)]
struct Node {
    item_type: ItemType,
    size: u32,
    children: Option<Vec<Option<Box<Node>>>>,
    parent: Option<Box<Node>>,
}
impl Node {}

fn sum_dir(curr: Node) {}

pub fn sum_sizes_of_directories() -> u32 {
    let mut file = File::open("utils/day_7_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let lines = contents.split("\n").collect::<Vec<&str>>();

    let mut file_tree = FileTree::new();

    let mut curr_dir: &mut Option<Box<Node>> = &mut None;
    //build the tree
    for line in lines.iter() {
        if line.len() < 3 {
            continue;
        }

        match &line[0..4] {
            "$ cd" => change_dir(&mut file_tree, curr_dir, &line[5..]),
            "$ ls" => handle_ls(curr_dir),
            _ => handle_item(&line),
        }
        println!("curr_dir : {:?}", &curr_dir)
    }

    return 0;
}
fn change_dir( file_tree: &mut FileTree, mut curr_dir: &mut Option<Box<Node>>, dir: &str) {
    if dir == "/" {
        curr_dir = &mut file_tree.root;
        return;
    }
    println!("{}", dir);
}
fn handle_ls(curr: &mut Option<Box<Node>>) {}

fn handle_item(listed_line: &str) {
    let details = listed_line.split(" ").collect::<Vec<&str>>();

    match details[0] {
        "dir" => (),
        _ => (),
    }
}
