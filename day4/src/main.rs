use std::io::{self, Write};
use std::ops::Range;
use std::ptr::copy;
use std::str::FromStr;
use std::sync::atomic::fence;
use std::{fs::File, io::BufRead};

use itertools::Itertools;

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

fn do_task_one(inputs: &Vec<Vec<u32>>) {
    let mut result = 0;
    for number in inputs {
        let mut a = &1;
        let mut ai = 0;
        let mut b = &1;
        let mut bi = 0;
        for (i, c) in number.iter().enumerate() {
            if c > a && i < (number.len() - 1) {
                a = c;
                ai = i;
                b = &0;
                bi = 0;
            }
            if (c > b && c <= a && i > ai) || c >= a && i == (number.len() - 1) {
                b = c;
                bi = i;
            }
        }
        println!("{:?} {:?} {:?} {:?}", a, ai, b, bi);
        result += (a * 10 + b);
    }
    println!("result {:?}", result);
}

fn rec(number: &[u32], numbers: &mut Vec<u32>) {
    let mut n = &0;
    let mut ni = 0;
    for (i, c) in number.iter().enumerate() {
        if i <= (number.len() - (12 - numbers.len())) && c > &n {
            n = &c;
            ni = i;
        }
    }
    numbers.push(*n);
    if numbers.len() < 12 {
        rec(&number[ni + 1..], numbers);
    }
}

fn do_task_two(inputs: &Vec<Vec<u32>>) {
    let mut result = 0;

    for number in inputs {
        let mut numbers: Vec<u32> = Vec::new();
        let mut n = &0;
        let mut ni = 0;
        for (i, c) in number.iter().enumerate() {
            if i <= (number.len() - (12 - numbers.len())) && c > &n {
                n = &c;
                ni = i;
            }
        }
        numbers.push(*n);
        rec(&number[ni + 1..], &mut numbers);
        let mut res: String = "".to_owned();
        numbers
            .iter()
            .enumerate()
            .for_each(|(i, n)| res += &n.to_string());
        result += res.parse::<u64>().unwrap();
        println!("{:?}", res.parse::<u64>());
    }
    println!("result {:?}", result);
}

fn main() {
    let inputs = read_from_file();
    do_task_two(&inputs);
    // println!("{:?}", &inputs);
}
