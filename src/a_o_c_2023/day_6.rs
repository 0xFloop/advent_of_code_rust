use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::ops::Index;
use std::str::FromStr;

#[derive(Debug)]
struct RaceData {
    alloted_time: u64,
    prev_record: u64,
}
pub fn solve_part_1() -> Option<u64> {
    let mut file = File::open("utils/2023/day_6_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let nums_regex = Regex::new(r"\b\d+\b").unwrap();

    let times_str = lines[0];

    let mut times: Vec<u64> = Vec::new();
    for time in nums_regex.find_iter(times_str) {
        match u64::from_str(time.as_str()) {
            Ok(num) => times.push(num),
            Err(e) => println!("Error: {}", e),
        }
    }

    let prev_record_distances_str = lines[1];

    let mut distances: Vec<u64> = Vec::new();

    for time in nums_regex.find_iter(prev_record_distances_str) {
        match u64::from_str(time.as_str()) {
            Ok(num) => distances.push(num),
            Err(e) => println!("Error: {}", e),
        }
    }

    let mut num_of_wins = 1;

    for i in 0..times.len() {
        let curr_race = RaceData {
            alloted_time: times[i],
            prev_record: distances[i],
        };

        let mut prev_record_release_time = 0;

        for i in 0..curr_race.alloted_time {
            let distance_achieved = i * (curr_race.alloted_time - i);
            if distance_achieved > curr_race.prev_record {
                prev_record_release_time = i - 1;
                break;
            } else if distance_achieved == curr_race.prev_record {
                prev_record_release_time = i;
                break;
            }
        }
        let num_of_beating_releases = curr_race.alloted_time - prev_record_release_time * 2 - 1;

        num_of_wins = num_of_wins * num_of_beating_releases;
    }
    return Some(num_of_wins);
}

pub fn solve_part_2() -> Option<u64> {
    let mut file = File::open("utils/2023/day_6_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let nums_regex = Regex::new(r"\b\d+\b").unwrap();

    let times_str = lines[0];

    let mut times: Vec<&str> = Vec::new();
    for time in nums_regex.find_iter(times_str) {
        times.push(time.as_str());
    }

    let prev_record_distances_str = lines[1];

    let mut distances: Vec<&str> = Vec::new();

    for distance in nums_regex.find_iter(prev_record_distances_str) {
        distances.push(distance.as_str());
    }
    let time_num = times.concat().parse::<u64>().unwrap();
    let distance_num = distances.concat().parse::<u64>().unwrap();

    let curr_race = RaceData {
        alloted_time: time_num,
        prev_record: distance_num,
    };

    let mut prev_record_release_time = 0;

    for i in 0..curr_race.alloted_time {
        let distance_achieved = i * (curr_race.alloted_time - i);
        if distance_achieved > curr_race.prev_record {
            prev_record_release_time = i - 1;
            break;
        } else if distance_achieved == curr_race.prev_record {
            prev_record_release_time = i;
            break;
        }
    }
    let num_of_beating_releases = curr_race.alloted_time - prev_record_release_time * 2 - 1;

    return Some(num_of_beating_releases);
}
