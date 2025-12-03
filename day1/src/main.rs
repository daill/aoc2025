use std::io::{self};
use std::{fs::File, io::BufRead};

fn read_from_file() -> Vec<(char, i32)> {
    let file = File::open("inputs");
    let result: Vec<(char, i32)> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: Vec<(char, i32)> = Vec::new();
            for line in lines {
                if let Ok(line) = line {
                    let splits = line.split_at(1);
                    result.push((
                        splits.0.chars().next().unwrap(),
                        splits.1.parse::<i32>().unwrap(),
                    ));
                    println!("{:?}", splits);
                }
            }
            result
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn do_task_one(inputs: &Vec<(char, i32)>) {
    let mut current = 50;
    let mut count = 0;
    inputs.iter().for_each(|(direction, cnt)| {
        match direction {
            'L' => current -= cnt,
            'R' => current += cnt,
            _ => panic!(""),
        };

        if current % 100 == 0 {
            count += 1;
        }

        print!("{:?}", count);
    });

    print!("-> {:?}", count);
}

fn do_task_two(inputs: &Vec<(char, i32)>) {
    let mut current: i32 = 50;
    let mut count = 0;
    inputs.iter().for_each(|(direction, cnt)| {
        match direction {
            'L' => {
                if current == 0 {
                    count += cnt / 100 as i32;
                } else if cnt >= &current {
                    count += 1 + (cnt - current) / 100 as i32;
                }
                current = (((current - cnt) % 100) + 100) % 100;
            }
            'R' => {
                count += (current + cnt) / 100 as i32;
                current = (current + cnt) % 100;
            }
            _ => panic!(""),
        };
    });

    println!("-> {:?}", count);
}

fn main() {
    let inputs = read_from_file();
    do_task_two(&inputs);
    // println!("{:?}", &inputs);
}
