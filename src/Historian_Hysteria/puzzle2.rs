use std::collections::HashSet;
use std::fs::File;
use std::io::{self,BufRead};
use std::collections::HashMap;

pub fn solve() -> io::Result<()>{
    let file_path = "Historian_Hysteria/input.txt";
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

    let mut countRight = HashMap::new();

    for &num in &right{
        *countRight.entry(num).or_insert(0) += 1;
    }

    let mut total = 0;
    for &num in &left{
        let num_score = countRight.get(&num).copied().unwrap_or(0);
        total += num_score * num;
    }
    
    println!("Similarity score is: {}",total);
    Ok(())
}