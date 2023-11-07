use std::fs::File;
use std::io::{self, Read};

fn main() {
    
}


fn calorie_counter() ->  u32 {
    let mut file = File::open("utils/calorieCounterInput.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let mut three_current_highest_calories:[u32;3] = [0,0,0];

    let  words: Vec<&str> = contents.split('\n').collect();

    let mut current_calories = 0u32;

    for word in words{        
        if word == ""{
            let smallest_current_highest_calories = smallest_array_element(three_current_highest_calories);

            if(current_calories > smallest_current_highest_calories.1){
                three_current_highest_calories[smallest_current_highest_calories.0] = current_calories;
            }
            current_calories = 0;
        } else {
            current_calories = current_calories + word.parse::<u32>().unwrap();

        }
    }

    return three_current_highest_calories.iter().sum();
}

fn smallest_array_element(array:[u32;3]) -> (usize, u32){
    let mut smallest = (0usize,array[0]);
    for value in array.iter().enumerate(){
        if *value.1 < smallest.1{
            smallest = (value.0, *value.1);
        }
    }
    return smallest;
}