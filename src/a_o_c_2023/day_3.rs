use std::fs::File;
use std::io::{self, Read};

pub fn solve_part_2() -> Option<u32> {
    //shown. 12 red cubes, 13 green cubes, and 14 blue cubes
    let mut file = File::open("utils/2023/day_3_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let lines: Vec<_> = contents.lines().collect();

    let mut sum_of_gears = 0;

    for i in 0..lines.len() {
        //loop through each line

        let mut line = lines[i].to_owned();
        //this prevents having to write additional logic for nums at end of line
        line.push('.');

        //loop through each char in the line
        for j in 0..line.len() {
            let char = line.chars().nth(j)?;
            if char == '*' {
                //if the char is a * then we need to search around it for connected numbers
                let gear_product = search_star(&lines, i, j);
                match gear_product {
                    Ok(x) => sum_of_gears = sum_of_gears + x,
                    Err(x) => panic!(),
                }
            }
        }
    }
    Some(sum_of_gears)
}
fn search_star(lines: &Vec<&str>, line_idx: usize, star_idx: usize) -> Result<u32, anyhow::Error> {
    //load the prior line if it exists
    let prev_line = lines.get(line_idx - 1);

    //load in the current line
    let curr_line = lines[line_idx];
    //load in the next line if it exists
    let next_line = lines.get(line_idx + 1);

    let mut connected_nums: Vec<u32> = Vec::new();
    //loop through each of the three lines
    for line_option in [prev_line, Some(&curr_line), next_line] {
        if line_option.is_none() {
            continue;
        }

        let line = line_option.unwrap();

        let mut nums: Vec<char> = Vec::new();

        //this tracker tells us where in relation to the star we found the number
        let mut char_num: i32 = -1;

        let star_as_i32: i32 = star_idx.try_into()?;

        for char in line.chars().skip(star_idx - 1).take(3) {
            if char.is_ascii_digit() {
                nums.push(char);

                let right_char = line
                    .chars()
                    .nth((char_num + star_as_i32 + 1) as usize)
                    .unwrap();

                if right_char.is_ascii_digit() {
                    nums.push(right_char);
                    let second_right_char = line.chars().nth((char_num + star_as_i32 + 2) as usize);
                    if second_right_char.is_some() && second_right_char.unwrap().is_ascii_digit() {
                        nums.push(second_right_char.unwrap());
                    }
                }
                //issue here
                let left_char = line.chars().nth((char_num + star_as_i32 - 1) as usize);

                if left_char.is_some() && left_char.unwrap().is_ascii_digit() {
                    nums.insert(0, left_char.unwrap());

                    let second_left_char = line.chars().nth((char_num + star_as_i32 - 2) as usize);
                    if second_left_char.is_some() && second_left_char.unwrap().is_ascii_digit() {
                        nums.insert(0, second_left_char.unwrap());
                    }
                }

                let num_converted: String = nums.clone().into_iter().collect();
                let num_u32: u32 = num_converted.parse()?;
                if !connected_nums.contains(&num_u32) {
                    connected_nums.push(num_u32);
                }
                nums.clear();
            }
            char_num += 1;
        }
    }

    if connected_nums.len() == 2 {
        return Ok(connected_nums[0] * connected_nums[1]);
    }
    return Ok(0);
}

pub fn solve_part_1() -> Option<u32> {
    //shown. 12 red cubes, 13 green cubes, and 14 blue cubes
    let mut file = File::open("utils/2023/day_3_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let lines: Vec<_> = contents.lines().collect();

    let mut sum_of_parts = 0;

    for i in 0..lines.len() {
        let mut line = lines[i].to_owned();
        //this prevents having to write additional logic for nums at end of line
        line.push('.');
        let mut num_idx_holder: Vec<usize> = Vec::new();

        for j in 0..line.len() {
            let char = line.chars().nth(j)?;
            let mut idx_adj_to_sym: bool;

            if char.is_ascii_digit() {
                num_idx_holder.push(j);
            } else if &num_idx_holder.len() > &0 {
                println!("{:?}", num_idx_holder);
                for idx in &num_idx_holder {
                    idx_adj_to_sym = search_idx(&lines, i, idx);
                    if idx_adj_to_sym {
                        let num_str: String = num_idx_holder
                            .iter()
                            .map(|&char_idx| line.chars().nth(char_idx).unwrap())
                            .collect();
                        println!("{num_str}");
                        let num = num_str.parse::<u32>().expect("error parsing num_str");

                        sum_of_parts = sum_of_parts + num;

                        break;
                    }
                }
                num_idx_holder.clear();
            }
        }
    }
    Some(sum_of_parts)
}

fn search_idx(lines: &Vec<&str>, line_idx: usize, char_idx: &usize) -> bool {
    let prev_line = lines.get(line_idx - 1);

    let curr_line = lines[line_idx];

    let next_line = lines.get(line_idx + 1);

    let char_amt_to_sub;
    let char_amt_to_add;

    if char_idx == &0 {
        char_amt_to_sub = 0;
    } else {
        char_amt_to_sub = 1;
    }
    // println!("char_idx: {char_idx}, {}",curr_line.len()-1);
    //one extra so that our bytes indexing is inclusive

    if char_idx == &(curr_line.len()) {
        char_amt_to_add = 1
    } else {
        char_amt_to_add = 2;
    }

    for line_option in [prev_line, Some(&curr_line), next_line] {
        if line_option.is_none() {
            continue;
        }

        let line = line_option.unwrap();

        let start_char_index = char_idx - char_amt_to_sub; // starting from 'w'
        let end_char_index = char_idx + char_amt_to_add; // ending at 'd'

        let start_byte_index = line
            .char_indices()
            .nth(start_char_index)
            .map(|(idx, _)| idx)
            .unwrap_or_else(|| line.len());

        let end_byte_index = line
            .char_indices()
            .nth(end_char_index)
            .map(|(idx, _)| idx)
            .unwrap_or_else(|| line.len());

        for char in line[start_byte_index..end_byte_index].chars() {
            if !char.is_ascii_digit() && char != '.' {
                return true;
            }
        }
    }
    return false;
}
