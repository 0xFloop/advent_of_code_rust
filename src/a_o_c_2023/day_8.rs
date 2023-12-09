use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::ops::Index;
use num::integer::lcm;

#[derive(Debug)]
struct MapNode {
    value: String,
    right: String,
    left: String,
}

pub fn solve_part_1() -> Option<u32> {
    let mut file = File::open("utils/2023/day_8_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let mut lines: Vec<_> = contents.lines().collect();

    let mut node_map: HashMap<String, MapNode> = HashMap::new();
    let directions = lines.remove(0);
    lines.remove(0);
    for line in &mut lines {
        let mut_line = line.replace("(", "").replace(")", "").replace(" ", "");

        let node_data: Vec<&str> = mut_line.split("=").collect();
        let node_val = node_data.get(0).unwrap();
        let children = node_data.get(1).unwrap().split(",").collect::<Vec<&str>>();
        let left_val = children.get(0).unwrap();
        let right_val = children.get(1).unwrap();

        node_map.insert(
            node_val.to_string(),
            MapNode {
                value: node_val.to_string(),
                right: right_val.to_string(),
                left: left_val.to_string(),
            },
        );
    }
    let mut curr_node = node_map.get("AAA").unwrap();
    let mut steps_to_find = 0;

    while curr_node.value != "ZZZ" {
        let curr_direction = directions
            .chars()
            .nth(steps_to_find % directions.len())
            .unwrap();
        if curr_direction == 'R' {
            curr_node = node_map.get(&curr_node.right).unwrap();
        } else {
            curr_node = node_map.get(&curr_node.left).unwrap();
        }
        steps_to_find += 1;
    }

    Some(steps_to_find as u32)
}

pub fn solve_part_2() -> Option<u128> {

    let mut file = File::open("utils/2023/day_8_input.txt").expect("File not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let mut lines: Vec<_> = contents.lines().collect();

    let mut node_map: HashMap<String, MapNode> = HashMap::new();
    let directions = lines.remove(0);
    lines.remove(0);

    let mut nodes_ending_in_A: Vec<String> = Vec::new();

    for line in &mut lines {
        let mut_line = line.replace("(", "").replace(")", "").replace(" ", "");

        let node_data: Vec<&str> = mut_line.split("=").collect();
        let node_val = node_data.get(0).unwrap();
        let children = node_data.get(1).unwrap().split(",").collect::<Vec<&str>>();
        let left_val = children.get(0).unwrap();
        let right_val = children.get(1).unwrap();

        let new_node = MapNode {
            value: node_val.to_string(),
            right: right_val.to_string(),
            left: left_val.to_string(),
        };

        node_map.insert(node_val.to_string(), new_node);

        if node_val.ends_with('A') {
            nodes_ending_in_A.push(node_val.to_string());
        }
    }
    let mut curr_nodes: Vec<&MapNode> = Vec::new();

    for node_val in &nodes_ending_in_A {
        curr_nodes.push(node_map.get(node_val).unwrap());
    }
    let mut first_end_nodes: Vec<(&MapNode, char)> = Vec::new();

    for node in curr_nodes {
        let mut steps_to_find = 0;
        let mut curr_node: &MapNode = node;

        while !curr_node.value.ends_with('Z') {
            let curr_direction = directions
                .chars()
                .nth(steps_to_find % directions.len())
                .unwrap();
            steps_to_find += 1;

            if curr_direction == 'R' {
                let next_node = node_map
                    .get(&curr_node.right)
                    .expect("can not find right node");
                curr_node = next_node;
            } else if curr_direction == 'L' {
                let next_node = node_map
                    .get(&curr_node.left)
                    .expect("can not find left node");
                curr_node = next_node;
            }
            if curr_node.value.ends_with('Z'){
                first_end_nodes.push((&curr_node,curr_direction));
                break;
            }
        }
    }
    let mut loop_lengths:Vec<u32> = Vec::new();

    for (end_node, end_direction) in first_end_nodes {
        let mut steps_to_find = 0;
        let mut curr_node: &MapNode = end_node;
        let original_val = &end_node.value;
        let origingal_dir = end_direction;       
        while true {
            let curr_direction = directions
                .chars()
                .nth(steps_to_find % directions.len())
                .unwrap();
            steps_to_find += 1;

            if curr_direction == 'R' {
                let next_node = node_map
                    .get(&curr_node.right)
                    .expect("can not find right node");
                curr_node = next_node;
            } else if curr_direction == 'L' {
                let next_node = node_map
                    .get(&curr_node.left)
                    .expect("can not find left node");
                curr_node = next_node;
            }
            if &curr_node.value == original_val && origingal_dir == curr_direction{
                break;
            }
        }
        loop_lengths.push(steps_to_find as u32);
    }
    let mut curr_lcm:u128 = 0;
    for lengths in loop_lengths.chunks(2) {
        if curr_lcm == 0 {
            curr_lcm = lcm(lengths[0] as u128, lengths[1] as u128);
        }else{
        curr_lcm = lcm(curr_lcm, lcm(lengths[0] as u128, lengths[1] as u128));
        }
    }
    Some(curr_lcm)
}
