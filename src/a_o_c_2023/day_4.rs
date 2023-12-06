use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::ops::Index;

pub fn solve_part_1() -> Option<u32> {
    //shown. 12 red cubes, 13 green cubes, and 14 blue cubes
    let mut file = File::open("utils/2023/day_4_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let lines: Vec<_> = contents.lines().collect();
    let mut win_sum = 0;
    for line in lines {
        let winners: Vec<&str> = Vec::new();

        let parts: Vec<&str> = line.split("|").collect();
        let game_details = parts[0];
        let winning_nums: Vec<&str> = game_details.split(":").collect::<Vec<&str>>()[1]
            .split(" ")
            .collect();
        let game_number = game_details.split(":").collect::<Vec<&str>>()[0]
            .split(" ")
            .collect::<Vec<&str>>()[1];

        let my_nums: Vec<&str> = parts[1].split(" ").collect();

        println!("{:?}\n{:?}", winning_nums, my_nums);

        let mut win_amt = 0;
        for num in my_nums {
            if num == "" {
                continue;
            }
            if winning_nums.contains(&num) {
                println!("winning num: {num}");
                if win_amt == 0 {
                    win_amt = 1;
                } else {
                    win_amt = win_amt * 2;
                }
            }
        }
        win_sum += win_amt;
    }

    return Some(win_sum);
}
pub fn solve_part_2() -> Option<u32> {
    //shown. 12 red cubes, 13 green cubes, and 14 blue cubes
    let mut file = File::open("utils/2023/day_4_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let lines: Vec<_> = contents.lines().collect();

    let num_of_games = lines.len();

    let mut num_of_winners_in_line: HashMap<u32, u32> = HashMap::new();

    // num of winners is an array where idx = gameid - 1 and value is the number of winners that game has
    let mut count_of_each_card: HashMap<u32, u32> = HashMap::new();

    for (idx, line) in lines.iter().enumerate() {
        let winners: Vec<&str> = Vec::new();

        let parts: Vec<&str> = line.split("|").collect();
        let game_details = parts[0];
        let winning_nums: Vec<&str> = game_details.split(":").collect::<Vec<&str>>()[1]
            .split(" ")
            .collect();
        let game_number = game_details.split(":").collect::<Vec<&str>>()[0]
            .split(" ")
            .collect::<Vec<&str>>()[1];

        let my_nums: Vec<&str> = parts[1].split(" ").collect();

        let mut num_of_winners = 0;
        for num in my_nums {
            if num == "" {
                continue;
            }

            if winning_nums.contains(&num) {
                num_of_winners += 1;
            }
        }
        num_of_winners_in_line.insert(idx as u32 + 1, num_of_winners);
    }
    println!("{:?}", num_of_winners_in_line);

    for i in 0..num_of_games {
        count_of_each_card.insert(i as u32 + 1, 1);
    }

    for game_id in 1..=num_of_games {
        //get current game card count
        let curr_game_count = count_of_each_card.entry(game_id as u32).or_insert(1);
        println!(
            "gameId: {game_id}, num of winners: {:?}",
            num_of_winners_in_line.get(&(game_id as u32))
        );

        //for the number of cards of current game, increment all games in range game_id..game_id+num_of_cards by 1

        //loop through count of curr game
        for j in 0..*curr_game_count {
            //loop though each next game based on number of winners game_id has
            for i in 1..=*num_of_winners_in_line.get(&(game_id as u32)).unwrap() {
                let sub_count = count_of_each_card.get(&(game_id as u32 + i));

                if sub_count.is_none() {
                    count_of_each_card.insert(game_id as u32 + i, 1);
                } else {
                    count_of_each_card.insert(game_id as u32 + i, sub_count.unwrap() + 1);
                }

                println!(
                    "incrementing game: {}, new card count: {}",
                    game_id as u32 + i,
                    count_of_each_card.get(&(game_id as u32 + i)).unwrap()
                );
                // count_of_each_card.insert(idx as u32 + i, curr_card_count.unwrap() + 1);
            }
        }
    }
    println!("{:?}", count_of_each_card);
    let mut sum_of_cards = 0;
    for (key, value) in count_of_each_card {
        sum_of_cards += value;
    }

    return Some(sum_of_cards);
}
