use num::integer::lcm;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::ops::Index;

pub fn solve_part_1() -> Option<i32> {
    let mut file = File::open("utils/2023/day_9_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let mut lines: Vec<_> = contents.lines().collect();

    let mut sum_of_history_vals = 0;

    for line in lines {
        let history = line.split(" ").collect::<Vec<&str>>();
        let mut history_nums: Vec<Vec<i32>> = Vec::new();
        let mut initial_history_nums: Vec<i32> = Vec::new();
        for num in history {
            initial_history_nums.push(num.parse::<i32>().unwrap());
        }
        history_nums.push(initial_history_nums);
        let mut history_idx = 0;
        while true {
            let mut new_history_nums: Vec<i32> = Vec::new();
            let curr_history_nums = history_nums.get(history_idx).unwrap();
            for i in 0..(curr_history_nums.len() - 1) {
                new_history_nums.push(
                    curr_history_nums.get(i + 1).unwrap() - curr_history_nums.get(i).unwrap(),
                );
            }

            if new_history_nums.iter().all(|&x| x == 0) {
                break;
            }
            history_nums.push(new_history_nums);
            history_idx += 1;
        }
        history_nums.reverse();
        let mut adder = 0;
        //this is the part one solution
        // for history in &history_nums {
        //     adder = adder + history.last().unwrap();
        // }
        //this is the part two solution
        for history in &history_nums {
            adder = history.first().unwrap() - adder;
        }
        sum_of_history_vals += adder;
    }
    Some(sum_of_history_vals)
}
