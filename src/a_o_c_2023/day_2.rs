use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

pub fn solve_part_1() -> Option<u32> {
    //shown. 12 red cubes, 13 green cubes, and 14 blue cubes
    let mut file = File::open("utils/2023/day_2_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let games: Vec<_> = contents.lines().collect();

    let mut valid_game_id_sum: u32 = 0;

    for game in games {
        let mut curr_game = Game {
            id: 999,
            red: 0,
            green: 0,
            blue: 0,
        };

        //split game by spaces (" "), scan vec for color, when you find, add prior number to that games corresponding color total
        let game_info: Vec<&str> = game.split(" ").collect();
        println!("{:?}", game_info);

        for i in 0..(game_info.len()) {
            let item = game_info[i];
            if i == 0 {
                curr_game.id = game_info[1]
                    .replace(":", "")
                    .parse()
                    .expect("error parsing game id");
            }

            if item.contains("red") {
                if curr_game.red
                    < game_info[i - 1]
                        .parse::<u32>()
                        .expect("error parsing red value")
                {
                    curr_game.red = game_info[i - 1].parse::<u32>().expect("error parsing red");
                    println!("color: red, number: {}", game_info[i - 1]);
                }
            } else if item.contains("green") {
                if curr_game.green
                    < game_info[i - 1]
                        .parse::<u32>()
                        .expect("error parsing green value")
                {
                    curr_game.green = game_info[i - 1]
                        .parse::<u32>()
                        .expect("error parsing green");
                    println!("color: green, number: {}", game_info[i - 1]);
                }
            } else if item.contains("blue") {
                if curr_game.blue
                    < game_info[i - 1]
                        .parse::<u32>()
                        .expect("error parsing blue value")
                {
                    curr_game.blue = game_info[i - 1].parse::<u32>().expect("error parsing blue");
                    println!("color: blue, number: {}", game_info[i - 1]);
                }
            }
        }

        if curr_game.red <= 12 && curr_game.blue <= 14 && curr_game.green <= 13 {
            println!("valid game");

            valid_game_id_sum += curr_game.id;
        } else {
            // println!("invalid game");
        }
    }
    return Some(valid_game_id_sum);
}

pub fn solve_part_2() -> Option<u32> {
    //shown. 12 red cubes, 13 green cubes, and 14 blue cubes
    let mut file = File::open("utils/2023/day_2_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let games: Vec<_> = contents.lines().collect();

    let mut game_product_sum: u32 = 0;

    for game in games {
        let mut curr_game = Game {
            id: 999,
            red: 0,
            green: 0,
            blue: 0,
        };

        //split game by spaces (" "), scan vec for color, when you find, add prior number to that games corresponding color total
        let game_info: Vec<&str> = game.split(" ").collect();
        println!("{:?}", game_info);

        for i in 0..(game_info.len()) {
            let item = game_info[i];
            if i == 0 {
                curr_game.id = game_info[1]
                    .replace(":", "")
                    .parse()
                    .expect("error parsing game id");
            }

            if item.contains("red") {
                if curr_game.red
                    < game_info[i - 1]
                        .parse::<u32>()
                        .expect("error parsing red value")
                {
                    curr_game.red = game_info[i - 1].parse::<u32>().expect("error parsing red");
                    println!("color: red, number: {}", game_info[i - 1]);
                }
            } else if item.contains("green") {
                if curr_game.green
                    < game_info[i - 1]
                        .parse::<u32>()
                        .expect("error parsing green value")
                {
                    curr_game.green = game_info[i - 1]
                        .parse::<u32>()
                        .expect("error parsing green");
                    println!("color: green, number: {}", game_info[i - 1]);
                }
            } else if item.contains("blue") {
                if curr_game.blue
                    < game_info[i - 1]
                        .parse::<u32>()
                        .expect("error parsing blue value")
                {
                    curr_game.blue = game_info[i - 1].parse::<u32>().expect("error parsing blue");
                    println!("color: blue, number: {}", game_info[i - 1]);
                }
            }
        }


        let curr_game_sum = curr_game.blue * curr_game.red * curr_game.green;


        game_product_sum += curr_game_sum;
    }
    return Some(game_product_sum);
}
