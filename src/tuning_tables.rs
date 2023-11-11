use std::{fs::File, io::Read};

pub fn get_num_of_chars() -> u32 {
    let mut file = File::open("utils/day_6_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let mut num_of_chars_before_marker = 0;
    let mut char_tracker: Vec<char> = vec!['0','0','0','0','0','0','0','0','0','0','0','0','0','0'];

    for i in 0..contents.len() {
        let chars = contents.chars().collect::<Vec<char>>();
        let char = chars[i];

        char_tracker.remove(0);
        char_tracker.push(char);
        let mut all_chars_are_unique = true;
        for char in &char_tracker {
            let num_of_times_in_array = char_tracker.iter().filter(|&c| c == char).count();
            if num_of_times_in_array > 1 {
                all_chars_are_unique = false;
                break;
            }
        } 

        if all_chars_are_unique {
            println!("{:?}",char_tracker);
            return (i as u32)+1;
        }
    }
    return 0;
}