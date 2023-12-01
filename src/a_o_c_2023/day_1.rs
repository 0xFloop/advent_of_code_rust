use core::num;
use std::env::join_paths;
use std::fs::File;
use std::io::{self, Read};

pub fn solve_part_1() -> u32 {
    let mut file = File::open("utils/2023/day_1_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let lines: Vec<_> = contents.split("\n").collect();

    let mut curr_sum: u32 = 0;
    for line in lines {
        let mut first_num: Option<char> = None;
        let mut last_num: Option<char> = None;
        for char in line.chars() {
            if char.is_numeric() {
                first_num = Some(char);
                break;
            }
        }
        for char in line.chars().rev() {
            if char.is_numeric() {
                last_num = Some(char);
                break;
            }
        }
        if first_num == None || last_num == None {
            print!("invalid nub");
            return 0;
        }
        let result: u32 = format!("{}{}", first_num.unwrap(), last_num.unwrap())
            .parse()
            .expect("invalid number");

        curr_sum = curr_sum + result;
    }
    return curr_sum;
}

struct NumMatch<'a> {
    word: &'a str,
    num: &'a str,
}

pub fn solve_part_2() -> Result<u32, &'static str> {
    let mut file = File::open("utils/2023/day_1_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let mut lines: Vec<_> = contents.split("\n").map(|line| line.to_string()).collect();

    let numbers = vec![
        NumMatch {
            word: "one",
            num: "1",
        },
        NumMatch {
            word: "two",
            num: "2",
        },
        NumMatch {
            word: "three",
            num: "3",
        },
        NumMatch {
            word: "four",
            num: "4",
        },
        NumMatch {
            word: "five",
            num: "5",
        },
        NumMatch {
            word: "six",
            num: "6",
        },
        NumMatch {
            word: "seven",
            num: "7",
        },
        NumMatch {
            word: "eight",
            num: "8",
        },
        NumMatch {
            word: "nine",
            num: "9",
        },
    ];
    let mut curr_sum = 0;

    for line in lines.iter_mut() {
        let mut curr_chars = String::new();

        let mut num_one: Option<String> = None;
        let mut num_two: Option<String> = None;

        for char in line.chars() {
            curr_chars.push(char);

            for num_match in &numbers {
                if curr_chars.contains(num_match.word) {
                    num_one = Some(num_match.num.to_string());
                    break;
                } else if char.is_ascii_digit() {
                    num_one = Some(char.to_string());
                    break;
                }
            }
            if num_one.is_some() {
                break;
            }
        }
        curr_chars = String::new();

        for char in line.chars().rev() {
            curr_chars.push(char);

            for num_match in &numbers {
                if curr_chars
                    .chars()
                    .rev()
                    .collect::<String>()
                    .contains(num_match.word)
                {
                    num_two = Some(num_match.num.to_string());
                    break;
                } else if char.is_ascii_digit() {
                    num_two = Some(char.to_string());
                    break;
                }
            }
            if num_two.is_some() {
                break;
            }
        }

        if line == "" {
            break;
        }
        if num_one == None || num_two == None {
            print!("invalid num issue");
            return Err("error with some nums");
        }

        let result: u32 = format!("{}{}", num_one.unwrap(), num_two.unwrap())
            .parse()
            .expect("invalid number");
        println!(", {result}");
        curr_sum = curr_sum + result;
    }
    return Ok(curr_sum);
}
//old with temp line usage: 58119
//using String no temp: 52572
