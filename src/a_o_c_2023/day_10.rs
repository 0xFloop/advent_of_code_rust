use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, Read};
use std::ops::Index;

#[derive(Debug)]
struct PipeOption {
    char: char,
    x: i32,
    y: i32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new(x: i32, y: i32) -> Self {
        return Point { x, y };
    }
}
impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub fn solve() -> Option<u32> {
    let mut file = File::open("utils/2023/day_10_input.txt").expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let mut lines: Vec<_> = contents.lines().collect();

    let map = contents
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Point::new(x as i32, y as i32), c))
        })
        .collect::<HashMap<Point, char>>();

    let mut start_point = map.iter().find(|(p, &c)| c == 'S').unwrap().0;

    let options_from_start: [Point; 4] = [
        Point::new(start_point.x - 1, start_point.y),
        Point::new(start_point.x, start_point.y - 1),
        Point::new(start_point.x + 1, start_point.y),
        Point::new(start_point.x, start_point.y + 1),
    ];

    let mut barriers: HashSet<Point> = HashSet::new();

    let next = *start_point + Point::new(0, 1);

    let mut barriers = HashSet::from([*start_point, next]);
    let cycle_length = recurse_point_part_2(&map, *start_point, next, 0, &mut barriers);

    println!("part 1: cycle_length: {}", cycle_length);

    //take each point in the matrixk and recursively check its surroundings until it reaches the edge of the matrix.

    //mark all nodes and paths that reach the edge as so

    let mut inside_points: Vec<Point> = Vec::new();
    let tes_points : Vec<Point> = Vec::new();

    let points = map
        .iter()
        .filter(|(p, &c)| c == '.' || !barriers.contains(p))
        .map(|(p, _)| p)
        .collect::<Vec<_>>();

    let rights = HashSet::from(['-', 'F', 'L']);
    let lefts = HashSet::from(['-', '7', 'J']);

    //loop through all our non wall points
    for dot in points {
        let num_of_left_barriers = barriers
            .iter()
            .filter(|p| p.x == dot.x && p.y < dot.y && lefts.contains(&map[p]))
            .count();

        let num_of_right_barriers = barriers
            .iter()
            .filter(|p| p.x == dot.x && p.y < dot.y && rights.contains(&map[p]))
            .count();

        if num_of_left_barriers.min(num_of_right_barriers) % 2 == 1 {
            inside_points.push(*dot);
        }
    }

    for (line_idx, line) in lines.iter().enumerate() {
        for (char_idx, char) in line.chars().enumerate() {
            if barriers.contains(&Point::new(char_idx as i32, line_idx as i32)) {
                print!("{}", display(char));
            } else if inside_points.contains(&Point::new(char_idx as i32, line_idx as i32)) {
                print!("$");
            } else {
                print!("{char}");
            }
        }
        println!()
    }

    Some(inside_points.len() as u32)
}

fn recurse_point_part_2(
    map: &HashMap<Point, char>,
    prev: Point,
    curr: Point,
    cycle_length: usize,
    barriers: &mut HashSet<Point>,
) -> usize {
    let char = map[&curr];

    if char == 'S' {
        let cycle_length = cycle_length + 1;
        if cycle_length % 2 == 0 {
            return cycle_length / 2;
        }
        return cycle_length / 2 + 1;
    }

    let nexts = get_pipe(char);

    let next = nexts.iter().find(|&&p| p != prev - curr).unwrap();
    barriers.insert(curr + *next);

    recurse_point_part_2(map, curr, curr + *next, cycle_length + 1, barriers)
}

fn get_pipe(char: char) -> Vec<Point> {
    match char {
        '|' => vec![Point::new(0, -1), Point::new(0, 1)],
        '-' => vec![Point::new(-1, 0), Point::new(1, 0)],
        'L' => vec![Point::new(0, -1), Point::new(1, 0)],
        'J' => vec![Point::new(0, -1), Point::new(-1, 0)],
        '7' => vec![Point::new(0, 1), Point::new(-1, 0)],
        _ => vec![Point::new(0, 1), Point::new(1, 0)],
    }
}
fn display(c: char) -> &'static str {
    match c {
        'S' => "S",
        '|' => "│",
        '-' => "─",
        'L' => "└",
        'J' => "┘",
        '7' => "┐",
        'F' => "┌",
        _ => "·",
    }
}
