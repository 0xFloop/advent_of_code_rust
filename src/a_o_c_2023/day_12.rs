use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

struct Config {}
pub fn solve() -> Option<usize> {
    let mut file = File::open("utils/2023/day_12_input.txt").expect("file not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let lines = contents.lines().collect::<Vec<&str>>();

    let mut total = 0;
    let mut memo_map = HashMap::new();

    for line in &lines {
        let split = line.split(' ').collect::<Vec<&str>>();
        let mut config = split.first().unwrap().to_string();
        let mut nums = split
            .get(1)
            .unwrap()
            .split(',')
            .map(|char| char.to_string().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut new_config = String::from(&config);
        let mut new_nums = nums.clone();

        for i in 0..4 {
            new_config.push_str(&("?".to_owned() + &config));
            new_nums.append(&mut nums.clone());
        }
        println!("{new_config} {:?}", new_nums);
        total += search_config_with_memoization(new_config, &new_nums, &mut memo_map);
    }

    Some(total)
}

fn search_config_with_memoization(
    config: String,
    nums: &Vec<usize>,
    memo_map: &mut HashMap<(String, Vec<usize>), usize>,
) -> usize {
    if let Some(&v) = memo_map.get(&(config.clone(), nums.to_vec())) {
        println!("memo hit here");
        return v;
    }

    if config == "" {
        if nums.len() == 0 {
            return 1;
        }
        return 0;
    }

    if nums.is_empty() {
        if config.contains("#") {
            return 0;
        }
        memo_map.insert((config.clone(), nums.to_vec()), 1);
        return 1;
    }

    if config.len() < nums.iter().sum::<usize>() + (nums.len() - 1) {
        memo_map.insert((config, nums.to_vec()), 0);
        return 0;
    }
    let curr_char = config.chars().nth(0).unwrap();

    let mut solutions = 0;
    if curr_char == '.' {
        let solutions = search_config_with_memoization(
            config.chars().skip(1).collect::<String>(),
            nums,
            memo_map,
        );
        memo_map.insert((config[1..].to_string(), nums.to_vec()), solutions);
        return solutions;
    }

    //this means our current letter is being taken as non-functional
    if nums[0] <= config.len()
        && !config.get(..nums[0]).unwrap().contains('.')
        && (nums[0] == config.len() || config.chars().nth(nums[0]).unwrap() != '#')
    {
        solutions = search_config_with_memoization(
            config.chars().skip(nums[0] + 1).collect::<String>(),
            &nums[1..].to_vec(),
            memo_map,
        );
    }

    // at this point we knoew taht char == '?'
    //count all the optoinos if we treat it as a '.' by skipping it
    if curr_char == '?' {
        solutions += search_config_with_memoization(
            config.chars().skip(1).collect::<String>(),
            nums,
            memo_map,
        );
    }

    memo_map.insert((config, nums.to_vec()), solutions);
    return solutions;
}

fn search_config(config: &str, nums: &Vec<usize>) -> usize {
    if config == "" {
        if nums.len() == 0 {
            return 1;
        }
        return 0;
    }

    if nums.is_empty() {
        if config.contains("#") {
            return 0;
        }
        return 1;
    }

    let mut result = 0;

    if ".?".contains(config.chars().nth(0).unwrap()) {
        //this means our current letter is being taken as functional
        //call search with same nums, but take one char off the front
        result += search_config(&config.chars().skip(1).collect::<String>(), nums);
    }

    if "#?".contains(config.chars().nth(0).unwrap()) {
        //this means our current letter is being taken as non-functional
        if nums[0] <= config.len()
            && !config.get(..nums[0]).unwrap().contains('.')
            && (nums[0] == config.len() || config.chars().nth(nums[0]).unwrap() != '#')
        {
            result += search_config(
                &config.chars().skip(nums[0] + 1).collect::<String>(),
                &nums[1..].to_vec(),
            )
        }
    }
    return result;
}
