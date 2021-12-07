// Day 1

use std::fs;

pub fn main() {
     let input = fs::read_to_string("input/1.txt").expect("Unable to read file");
     let nums: Vec<u32> = input.lines().map(
         |line| line.parse::<u32>().unwrap() // parse needs to be typed with turbofish ::<>, and is a result that needs to be unwrapped
     ).collect(); // convert string to numbers
     
     // Part 1
     // for each pair see if second is greater than the first
     let pairs = nums.windows(2); // iterator for each pair of numbers for part 1
     let count1 = pairs.filter(|pair| pair[1] > pair[0]).count(); // map windows to comparisons, filter only trues and count
     
     // Part 2
     // for each triplet window see if second triplet sum is greater than the first
     let triplet_sums: Vec<u32> = nums.windows(3).map(|triplet| triplet[0] + triplet[1] + triplet [2]).collect(); // sum for each triplet of numbers for part 2
     let triplet_pairs = triplet_sums.windows(2); // iterator comparing pairs of triplet sums
     
     let count2 = triplet_pairs.filter(|trip| trip[1] > trip[0]).count(); // map windows to comparisons, filter only trues and count
     
     println!("{}, {}", count1, count2);
}