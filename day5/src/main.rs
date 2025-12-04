use std::collections::HashSet;
use std::io::{self};
use std::{fs::File, io::BufRead};

fn read_from_file() -> HashSet<(isize, isize)> {
    let file = File::open("inputs");
    let result: HashSet<(isize, isize)> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: HashSet<(isize, isize)> = HashSet::new();
            for (id, line) in lines.enumerate() {
                if let Ok(line) = line {
                    line.chars().enumerate().for_each(|(i, x)| {
                        if x == '@' {
                            let c = ((id as isize), (i as isize));
                            result.insert(c.clone());
                        }
                    });
                }
            }
            result
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn do_task_one(inputs: &HashSet<(isize, isize)>) {
    let mut result = process(&inputs);
    println!("{:?}", result.len());
}

fn main() {
    let inputs = read_from_file();
    do_task_two(&inputs);
    // println!("{:?}", &inputs);
}
