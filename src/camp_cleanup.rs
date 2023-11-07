use std::{fs::File, io::Read};

pub fn cleanup_the_camp() -> u32 {
    let mut file = File::open("utils/day_4_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let words: Vec<&str> = contents.split('\n').collect();

    let mut num_of_contained_pairs = 0u32;

    for word in words{
        if word == ""{
            continue;
        }
        let ranges:Vec<_> = word.split(",").collect();

        let range_one: Vec<_> = ranges[0].split("-").collect();
        let range_two: Vec<_> = ranges[1].split("-").collect();
        
        if range_one[0].parse::<u32>().unwrap() <= range_two[0].parse::<u32>().unwrap() && range_one[1].parse::<u32>().unwrap() >= range_two[1].parse::<u32>().unwrap(){
            print!("range 1 contains range 2");
            println!("{:?}, {:?}",range_one,range_two);
            num_of_contained_pairs+=1;
        } else if range_two[0].parse::<u32>().unwrap() <= range_one[0].parse::<u32>().unwrap() && range_two[1].parse::<u32>().unwrap() >= range_one[1].parse::<u32>().unwrap() {
            print!("range 2 contains range 1");
            println!("{:?}, {:?}",range_one,range_two);
            num_of_contained_pairs+=1;
        }
        
    }

    return num_of_contained_pairs;
}

pub fn cleanup_the_camp_part_two() ->u32 {
    let mut file = File::open("utils/day_4_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let words: Vec<&str> = contents.split('\n').collect();

    let mut num_of_overlapping_pairs = 0u32;

    for word in words{
        if word == ""{
            continue;
        }
        let ranges:Vec<_> = word.split(",").collect();

        let range_one: Vec<_> = ranges[0].split("-").collect();
        let range_two: Vec<_> = ranges[1].split("-").collect();
        
        if 
        range_one[0].parse::<u32>().unwrap() <= range_two[0].parse::<u32>().unwrap() && range_one[1].parse::<u32>().unwrap() >= range_two[0].parse::<u32>().unwrap() 
        || 
        range_one[0].parse::<u32>().unwrap() <= range_two[1].parse::<u32>().unwrap() && range_one[1].parse::<u32>().unwrap() >= range_two[1].parse::<u32>().unwrap(){
            print!("ther is overlap here");
            println!("{:?}, {:?}",range_one,range_two);
            num_of_overlapping_pairs+=1;
        } 
        
    }

    return num_of_overlapping_pairs;
}