use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::ops::Index;

pub fn solve_part_1() -> Option<u64> {
    let mut file = File::open("utils/2023/day_5_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let sections: Vec<&str> = contents.split("\n\n").collect();

    let seeds = sections[0].split(": ").collect::<Vec<&str>>()[1];

    let mut maps: Vec<Vec<&str>> = Vec::new();

    for section in sections {
        if section.contains("seeds:") {
            continue;
        }
        let mut section_lines = section.split("\n").collect::<Vec<&str>>();
        section_lines.remove(0);
        maps.push(section_lines);
    }

    let mut location_nums: Vec<u64> = Vec::new();

    for seed in seeds.split(" ").collect::<Vec<&str>>() {
        let mut seed_num: u64 = seed.parse().unwrap();

        for map in &maps {
            for line in map {
                let nums_in_line: Vec<&str> = line.split(" ").collect();
                let output = nums_in_line[0].parse::<u64>().unwrap();
                let input = nums_in_line[1].parse::<u64>().unwrap();
                let offset = nums_in_line[2].parse::<u64>().unwrap();

                if input <= seed_num && seed_num < input + offset {
                    seed_num = seed_num + output - input;
                    break;
                }
            }
        }
        location_nums.push(seed_num);
    }
    let mut smallest = u64::MAX;
    for num in location_nums {
        if num < smallest {
            smallest = num;
        }
    }
    return Some(smallest);
}

pub fn solve_part_2() -> Option<u64> {
    let mut file = File::open("utils/2023/day_5_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let sections: Vec<&str> = contents.split("\n\n").collect();

    let seed_ranges_str = sections[0].split(": ").collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>();

    let mut start_seed_ranges: Vec<Vec<u64>> = Vec::new();

    for range in seed_ranges_str.chunks(2) {
        let mut temp_range_vec: Vec<u64> = Vec::new();
        temp_range_vec.push(range[0].parse::<u64>().unwrap());
        temp_range_vec.push(temp_range_vec[0] + range[1].parse::<u64>().unwrap() - 1);

        start_seed_ranges.push(temp_range_vec);
    }

    let mut maps: Vec<Vec<&str>> = Vec::new();

    for section in sections {
        if section.contains("seeds:") {
            continue;
        }
        let section_lines = section.split("\n").collect::<Vec<&str>>();
        maps.push(section_lines);
    }

    let mut range_to_step_map: HashMap<Vec<u64>, u32> = HashMap::new();

    let smallest = recurse_ranges(start_seed_ranges, &maps, u64::MAX, &mut range_to_step_map);

    return Some(smallest);
}

fn recurse_ranges(
    ranges: Vec<Vec<u64>>,
    maps: &Vec<Vec<&str>>,
    mut curr_lowest: u64,
    range_to_step_map: &mut HashMap<Vec<u64>, u32>,
) -> u64 {
    for mut range in ranges {
        //each range is mapped to an index of the mutation map it was broken out from. this so so we dont
        //doubly apply mutations to a range. ie if [20,100] was broken out in step 4, when we go to analyze
        //that range we will start from step 4

        let range_to_map_idx = range_to_step_map.entry(range.clone()).or_insert_with(|| 0);
        let mut new_ranges: Vec<Vec<u64>> = Vec::new();

        for i in *range_to_map_idx..(maps.len() as u32) {
            let map = &maps[i as usize];
            for line in &map[1..] {
                let nums_in_line: Vec<&str> = line.split(" ").collect();
                let output = nums_in_line[0].parse::<u64>().unwrap();
                let input_val = nums_in_line[1].parse::<u64>().unwrap();
                let offset = nums_in_line[2].parse::<u64>().unwrap();
                let manip = [input_val, input_val + offset - 1];

                if manip[0] <= range[0] && range[1] <= manip[1] {
                    //range falls fully in the map manipulation range
                    //manipulate whole range values
                    range[0] = range[0] + output - manip[0];
                    range[1] = range[1] + output - manip[0];
                    break;
                } else if manip[0] > range[0] && range[1] >= manip[0] && range[1] <= manip[1] {
                    //range lower bound falls outside of map manipulation range
                    //split lower portion of range into seperate array and add that to our list of ranges
                    // apply manipulation to the remaining upper portion of range that falls within the manipulation
                    let range_lower_outside: Vec<u64> = vec![range[0], manip[0] - 1];

                    range = vec![manip[0], range[1]];
                    range[0] = range[0] + output - input_val;
                    range[1] = range[1] + output - input_val;

                    range_to_step_map.insert(range_lower_outside.clone(), i);
                    new_ranges.push(range_lower_outside);
                    break;
                } else if manip[0] <= range[0] && range[0] < manip[1] && range[1] > manip[1] {
                    //range outer bound falls outside of map manipulation range
                    //aplit the upper portion of the range into seperate array and add that to our list of ranges
                    //apply manipulation to the remaining lower portion of range that falls within the manipulation
                    let range_upper_outside = vec![manip[1] + 1, range[1]];

                    range = vec![range[0], manip[1]];
                    range[0] = range[0] + output - input_val;
                    range[1] = range[1] + output - input_val;

                    range_to_step_map.insert(range_upper_outside.clone(), i);
                    new_ranges.push(range_upper_outside);
                    break;
                } else if manip[0] >= range[0] && manip[1] <= range[1] {
                    //the manipulation range falls fully within our range
                    //split the range into 3 portions, pre manip range, manip range, and post manip range.
                    //apply mutation to manip range, push other two ranges into new_ranges
                    let range_lower_outside: Vec<u64> = vec![range[0], manip[1] - 1];
                    let range_upper_outside = vec![manip[1] + 1, range[1]];

                    range = vec![manip[0], manip[1]];
                    range[0] = range[0] + output - input_val;
                    range[1] = range[1] + output - input_val;

                    range_to_step_map.insert(range_lower_outside.clone(), i);
                    new_ranges.push(range_lower_outside);
                    range_to_step_map.insert(range_upper_outside.clone(), i);
                    new_ranges.push(range_upper_outside);
                    break;
                }
            }
        }
        if range[0] < curr_lowest {
            curr_lowest = range[0];
        }
        curr_lowest = recurse_ranges(new_ranges, &maps, curr_lowest, range_to_step_map);
    }
    return curr_lowest;
}
