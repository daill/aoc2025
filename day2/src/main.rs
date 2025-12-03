use std::io::{self, Write};
use std::{fs::File, io::BufRead};

fn read_from_file() -> Vec<(u64, u64)> {
    let mut file = File::open("inputs");
    let result: Vec<(u64, u64)> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: Vec<(u64, u64)> = Vec::new();
            for line in lines {
                if let Ok(line) = line {
                    let ranges: Vec<&str> = line.split(',').collect::<Vec<&str>>();
                    for range in ranges {
                        let splits: Vec<&str> = range.split('-').collect();
                        result.push((splits[0].parse().unwrap(), splits[1].parse().unwrap()));
                        println!("{:?}", splits);
                    }
                }
            }
            result
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn do_task_one(inputs: &Vec<(u64, u64)>) {
    let mut result: u64 = 0;
    for range in inputs {
        for i in range.0..=range.1 {
            //println!("{:?}", i);
            let parsed = i.to_string();
            let chars: Vec<char> = parsed.chars().collect();
            let c = chars.len();
            if &c % 2 == 0 {
                let mid = c / 2;
                if &chars[..mid] == &chars[mid..] {
                    //println!("invalid {:?}", i);
                    result += i;
                }
            }
        }
    }
    println!("{:?}", result);
}

fn do_task_two(inputs: &Vec<(u64, u64)>) {
    let mut result: u64 = 0;
    for range in inputs {
        for i in range.0..=range.1 {
            let parsed = i.to_string();
            let chars: Vec<char> = parsed.chars().collect();
            let c = chars.len();
            let mut valid = true;
            for j in 1..=c / 2 {
                //println!("-> {:?}", i);
                let sc = &chars[..j];
                let mut test = true;
                if c % j == 0 {
                    for k in (j..=c).step_by(j) {
                        if &chars[k - j..k] != sc {
                            //println!("valid {:?} {:?} {:?}", i, sc, &chars[k - j..k]);
                            test = false;
                            break;
                        }
                    }
                    if test {
                        valid = false;
                    }
                }
            }
            if !valid {
                println!("invalid {:?}", i);
                valid = true;
                result += i;
            }
        }
    }
    println!("{:?}", result);
}

fn main() {
    let inputs = read_from_file();
    do_task_two(&inputs);
    // println!("{:?}", &inputs);
}
