use std::fs::File;
use std::io::{self,BufRead, BufReader};

pub fn solve() -> io::Result<()>{
    let input_path = "Red_Nosed_Reports/input.txt";
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut safe = 0;

    for line in reader.lines(){
        let mut current_line = line?;
        let mut current_input : Vec<i32> = current_line
                                            .split_whitespace()
                                            .filter_map(|s| s.parse::<i32>().ok())
                                            .collect();

        if current_input[0] > current_input[1]{
            let mut i =0;
            while i < current_input.len() -1 {
                let diff = current_input[i] - current_input[i+1];
                if diff >= 1 && diff <= 3{
                    i += 1;
                }else {
                    break;
                }
            }
            if i == current_input.len() -1 {
                safe += 1;
            }
        }else if current_input[0] < current_input[1]{
            let mut i =1;
            while i < current_input.len() {
                let diff = current_input[i] - current_input[i-1];
                if diff >= 1 && diff <= 3{
                    i += 1;
                }else {
                    break;
                }
            }
            if i == current_input.len() {
                safe += 1;
            }
        }else{
            continue;
        }
    }

    println!("{}", safe);
    Ok(())
}