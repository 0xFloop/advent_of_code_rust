use std::{fs::File, io::Read};

pub fn get_top_crates() -> u32 {
    let mut file = File::open("utils/day_5_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let instructions: Vec<&str> = contents.split('\n').collect();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    let num_of_stacks = 9;

    for _ in 0..num_of_stacks {
        stacks.push(Vec::new());
    }

    let mut isolated_chars: Vec<char> = Vec::new();

    let mut final_letters: Vec<&char> = Vec::new();

    for instruction in &instructions {
        if instruction == &"" {
            continue;
        }
        if !instruction.starts_with("move") {
            //this is a setup stacks instruction
            if !(instruction.contains("1")) {
                for i in (0..instruction.len()).step_by(4) {
                    let instruction_chars: Vec<_> = instruction.chars().collect();
                    let current_char: &[char];

                    if i + 4 > instruction.len() {
                        current_char = &instruction_chars[i..instruction.len()];
                    } else {
                        current_char = &instruction_chars[i..i + 4];
                    }

                    isolated_chars.push(current_char[1]);
                }
                continue;
            }
        }
    }
    for i in 0..isolated_chars.len() {
        let current_isolated_char = isolated_chars[i];
        if current_isolated_char == ' ' {
            continue;
        }
        stacks[i % 9].push(current_isolated_char);
    }
    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    for instruction in instructions {
        if instruction == "" {
            continue;
        }
        if !instruction.starts_with("move") {
            continue;
        }
        let instruction_split: Vec<_> = instruction.split(" ").collect();

        let num_to_move = instruction_split[1].parse::<u32>().unwrap();
        let from_stack = instruction_split[3].parse::<u32>().unwrap() - 1;
        let to_stack = instruction_split[5].parse::<u32>().unwrap() - 1;

        for i in 0..num_to_move {
            let current_char = stacks[from_stack as usize].pop().unwrap();
            stacks[to_stack as usize].push(current_char);
        }


    }
    let mut final_letters: Vec<&char> = Vec::new();

    for stack in stacks.iter_mut() {
        final_letters.push(stack.last().unwrap());
    }
    println!("{:?}", final_letters);

    return 3;
}
