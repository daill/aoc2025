use std::io::{self, Write};
use std::{fs::File, io::BufRead};

fn read_from_file() -> Vec<Vec<u32>> {
    let mut file = File::open("inputs");
    let result: Vec<Vec<u32>> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: Vec<Vec<u32>> = Vec::new();
            for line in lines {
                if let Ok(line) = line {
                    let chars: Vec<u32> = line
                        .chars()
                        .map(|c| c.to_digit(10 as u32).unwrap())
                        .collect();
                    result.push(chars);
                }
            }
            result
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn do_task_one(inputs: &Vec<Vec<u32>>) {}

fn main() {
    let inputs = read_from_file();
    do_task_two(&inputs);
    // println!("{:?}", &inputs);
}
