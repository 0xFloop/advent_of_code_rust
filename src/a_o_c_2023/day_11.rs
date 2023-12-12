use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug, PartialEq, Eq)]
struct GalaxyPoint {
    x: i32,
    y: i32,
}
impl GalaxyPoint {
    fn new(x: i32, y: i32) -> Self {
        GalaxyPoint { x, y }
    }
    fn get_distance(&self, rhs: &Self) -> i32 {
        let y_dis = rhs.y - self.y;
        let x_dis = rhs.x - self.x;
        x_dis.abs() + y_dis.abs()
    }
}

pub fn solve() -> Option<i32> {
    let mut file = File::open("utils/2023/day_11_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let mut lines = contents.lines().collect::<Vec<&str>>();

    let mut expanded_universe: Vec<String> = Vec::new();

    let mut galaxy_map: HashMap<i32, GalaxyPoint> = HashMap::new();

    let mut galaxy_count = 0;

    for line in &lines {
        if line.contains('#') {
            expanded_universe.push(line.to_string())
        } else {
            expanded_universe.push(line.to_string());
            expanded_universe.push(line.to_string());
        }
    }

    let mut columns_expanded = 0;
    for i in 0..lines.get(0).unwrap().len() {
        let empty_column = lines.iter().all(|line| line.chars().nth(i).unwrap() != '#');
        if empty_column {
            expanded_universe
                .iter_mut()
                .for_each(|line| line.insert(i + columns_expanded, '.'));
            columns_expanded += 1;
        }
    }

    for (line_idx, line) in expanded_universe.iter().enumerate() {
        for (char_idx, char) in line.char_indices() {
            if char == '#' {
                galaxy_count += 1;
                print!("{galaxy_count}");
                galaxy_map.insert(
                    galaxy_count,
                    GalaxyPoint::new(char_idx as i32, line_idx as i32),
                );
            } else {
                print!(".");
            }
        }
        println!();
    }
    let mut sum_of_distances = 0;
    for i in 1..=galaxy_count {
        let curr_gal = galaxy_map.get(&i).unwrap();
        for j in i + 1..=galaxy_count {
            sum_of_distances += curr_gal.get_distance(galaxy_map.get(&j).unwrap());
        }
    }

    Some(sum_of_distances)
}
pub fn solve_part_2() -> Option<i64> {
    let mut file = File::open("utils/2023/day_11_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let mut lines = contents.lines().collect::<Vec<&str>>();

    let mut expanded_universe: Vec<String> = Vec::new();

    let mut galaxy_map: HashMap<i32, GalaxyPoint> = HashMap::new();

    let mut galaxy_count = 0;

    let mut expanded_row_idxs: Vec<usize> = Vec::new();
    let mut expanded_column_idxs: Vec<usize> = Vec::new();

    for (line_idx, line) in lines.iter().enumerate() {
        if !line.contains('#') {
            expanded_row_idxs.push(line_idx);
        }
    }

    for i in 0..lines.get(0).unwrap().len() {
        let empty_column = lines.iter().all(|line| line.chars().nth(i).unwrap() != '#');
        if empty_column {
            expanded_column_idxs.push(i);
        }
    }

    let mut row_expansions_passed = 0;
    for (line_idx, line) in lines.iter().enumerate() {
        if expanded_row_idxs.contains(&line_idx) {
            row_expansions_passed += 1;
            continue;
        }

        let mut column_expansions_passed = 0;
        for (char_idx, char) in line.char_indices() {
            if expanded_column_idxs.contains(&char_idx) {
                column_expansions_passed += 1;
                continue;
            }
            if char == '#' {
                galaxy_count += 1;
                galaxy_map.insert(
                    galaxy_count,
                    GalaxyPoint::new(
                        char_idx as i32 + (column_expansions_passed * 999999),
                        line_idx as i32 + (row_expansions_passed * 999999),
                    ),
                );
            }
        }
    }
    let mut sum_of_distances: i64 = 0;
    for i in 1..=galaxy_count {
        let curr_gal = galaxy_map.get(&i).unwrap();
        for j in i + 1..=galaxy_count {
            sum_of_distances += curr_gal.get_distance(galaxy_map.get(&j).unwrap()) as i64;
        }
    }
    Some(sum_of_distances)
}
// 1030 -> 1112
// 8410 -> 8492
