use std::{fs::File, io::Read, collections::HashMap};

pub fn sum_sizes_of_directories() -> u32 {
    let mut sum = 0;
    let mut file = File::open("utils/day_7_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    print!("{}", contents);

    let mut commands = contents.split("\n").collect::<Vec<&str>>();

    let mut current_directory = "";
    let mut dirs_to_size = HashMap::new();
    for i in 0..commands.len() {
        let command = commands[i];
        if command.contains("cd"){
            current_directory = command.split(" ").collect::<Vec<&str>>()[2];
        }
        else if command.starts_with("$") ||  {

        }
        println!("{}",command);
    }

    88
}