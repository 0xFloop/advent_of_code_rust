use std::{fs::File, io::Read};

pub fn reoraganize_the_rucksack()->u32{
    let mut file = File::open("utils/day_3_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let words: Vec<&str> = contents.split('\n').collect();

    let mut repeat_letters:Vec<char> = Vec::new();

    for word in words {
        if word ==""{
            continue;
        }

        let first_sack = &word[0..word.len()/2];
        let second_sack = &word[word.len()/2..word.len()];


        for char in first_sack.chars(){
            if second_sack.chars().any(|c| c == char){
                repeat_letters.push(char);
                break;
            }
        }
    }

    return calculate_sum_of_priorities(repeat_letters);
}

pub fn reoraganize_the_rucksack_part_two()->u32{
    let mut file = File::open("utils/day_3_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let words: Vec<&str> = contents.split('\n').collect();

    let mut repeat_letters:Vec<char> = Vec::new();

    for i in (0..words.len()-1 ).step_by(3){
        print!("{}",i);
        let word_one = words[i];
        let word_two = words[i+1];
        let word_three = words[i+2];



        for char in word_one.chars(){
            if word_two.chars().any(|c| c == char){
                if word_three.chars().any(|c| c == char){
                    repeat_letters.push(char);
                    break;
                }   
            }
        }
    }

    return calculate_sum_of_priorities(repeat_letters);
}

fn calculate_sum_of_priorities(repeat_letters:Vec<char>)->u32{
    let mut sum_of_priorities = 0u32;

    for mut letter in repeat_letters{
        let mut adjusted_ascii_number = 0;
        if letter.is_lowercase(){
            letter = letter.to_ascii_uppercase();
            let ascii_number = letter as u8;
            adjusted_ascii_number = ascii_number - 64;

        } else {
            letter = letter.to_ascii_lowercase();
            let ascii_number = letter as u8;
            adjusted_ascii_number = ascii_number - 70;
        }        

        sum_of_priorities += adjusted_ascii_number as u32;
        
    }

    return sum_of_priorities;
}