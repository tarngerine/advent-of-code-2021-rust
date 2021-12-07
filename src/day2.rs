// Day 2

use std::fs;

pub fn main() {
    let input = fs::read_to_string("input/2.txt").expect("Unable to read file");
    one(&input);
    println!("Part 2: {}", two(&input));
}

fn one(input: &String) {
     // sum all "forward" lines
     let horizontal_position: i32 = input.lines().map(
         |line| {
             if line.contains("forward") {
                 get_delta(&line)
             } else {
                 0
             }
         }
     ).sum();
     
     // sum all "up" and "down" lines
     let depth: i32 = input.lines().map(
         |line| {
             if line.contains("down") {
                 get_delta(&line) as i32
             } else if line.contains("up") {
                 get_delta(&line) as i32 * -1
             } else {
                 0
             }
         }
     ).sum();
    
     println!("{}, {}, {}", horizontal_position, depth, horizontal_position * depth);
}

// Create an enum for each direction that has stored values for the amount moved
enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn two(input: &String) -> i32 {
     // sum all "forward" lines, unchanged from part 1
     let directions: Vec<Direction> = input.lines().map(
         |line| {
             let dir = get_dir(&line);
             let delta = get_delta(&line);
             match dir {
                 "forward" => Direction::Forward(delta),
                 "down" => Direction::Down(delta),
                 "up" => Direction::Up(delta),
                 _ => panic!("invalid direction on line")
             }
         }
     ).collect();
     
     let mut x = 0;
     let mut y = 0;
     let mut aim = 0;
     
     // for each direction follow the rules of the prompt
     directions.iter().for_each(|dir| match dir {
         Direction::Forward(delta) => {
             x += delta;
             y += delta * aim;
         },
         Direction::Down(delta) =>  aim += delta,
         Direction::Up(delta) =>  aim -= delta,
     });
     
     x * y
}

// splits line of text and grabs the actual numerical value
fn get_delta(line: &str) -> i32 {
    line.split(' ').collect::<Vec<_>>()[1].parse::<i32>().unwrap()
}

// splits line of text and grabs the direction as string
fn get_dir(line: &str) -> &str { // has to be typed &str, not str, need to figure out why https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str
    line.split(' ').collect::<Vec<&str>>()[0]
}