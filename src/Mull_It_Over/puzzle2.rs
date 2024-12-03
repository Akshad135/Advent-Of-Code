use std::fs::File;
use std::io::{BufReader, Read};
use regex::Regex;
use std::error::Error;

pub fn solve() -> Result<(), Box<dyn Error>> {
    let path = "Mull_It_Over/input.txt";
    let file = File::open(path)?;

    let mut reader = BufReader::new(file);
    let mut mystr = String::new();
    reader.read_to_string(&mut mystr)?;

    let mut total = 0;
    let mut flag = true;

    let re = Regex::new(r"(don't|do)\(\)|mul\((\d{1,3}),(\d{1,3})\)")?;

    for mat in re.find_iter(&mystr) {
        let mymatch = mat.as_str();

        if mymatch.contains("don't()") {
            flag = false;
        } else if mymatch.contains("do()") {
            flag = true;
        }else if flag {
            let nums_re = Regex::new(r"\d{1,3}")?;
            let nums: Vec<i32> = nums_re
                .find_iter(mymatch)
                .filter_map(|s| s.as_str().parse::<i32>().ok())
                .collect();
            total += nums[0] * nums[1];
        }
    }

    println!("Adding all muls while enabled: {}", total);
    Ok(())
}