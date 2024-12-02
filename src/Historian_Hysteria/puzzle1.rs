use std::fs::File;
use std::io::{self,BufRead};

pub fn solve() -> io::Result<()>{
    let file_path = "Historian_Hysteria/puzzle.txt";
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in reader.lines(){
        let currentLine = line?;
        let nums: Vec<i32> = currentLine
                    .split_whitespace()
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect();

        left.push(nums[0]);
        right.push(nums[1]);
    }

    left.sort();
    right.sort();

    let mut total = 0;

    for i in 0..left.len(){
        let diff = left[i] - right[i];
        total += diff.abs();
    }

    println!("The total distance in both lists is: {}",total);
    Ok(())
}