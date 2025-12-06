use std::collections::HashSet;
use std::io::{self, Cursor};
use std::ops::RangeInclusive;
use std::{fs::File, io::BufRead};

fn read_from_file() -> (Vec<(u64, u64)>, Vec<u64>) {
    let file = File::open("inputs");
    let result: (Vec<(u64, u64)>, Vec<u64>) = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut ranges: Vec<(u64, u64)> = Vec::new();
            let mut ing: Vec<(u64)> = Vec::new();
            for (id, line) in lines.enumerate() {
                if let Ok(line) = line {
                    if line.len() != 0 {
                        if line.contains('-') {
                            let mut parts = line.split('-');
                            let d = parts.next().unwrap();
                            let u = parts.next().unwrap();
                            ranges.push((d.parse().unwrap(), u.parse().unwrap()));
                        } else {
                            ing.push(line.parse().unwrap());
                        }
                    }
                }
            }
            (ranges, ing)
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn do_task_two(inputs: &mut (Vec<(u64, u64)>, Vec<u64>)) {
    let mut ranges = inputs.0.clone();
    ranges.sort_by_key(|rang| (rang.0, rang.1));
    let mut last_range = ranges.first().unwrap();
    let mut res = last_range.1 - last_range.0;
    let mut las_top = last_range.1;
    for i in 1..ranges.len() {
        let cur_range = ranges.get(i).unwrap();
        if cur_range.1 > las_top && cur_range.0 < las_top {
            res += cur_range.1 - las_top;
        } else {
            if cur_range.0 >= last_range.0
                && cur_range.0 <= last_range.1
                && cur_range.1 > last_range.1
            {
                res += cur_range.1 - last_range.1;
            } else if cur_range.0 >= last_range.1 {
                if cur_range.0 == last_range.1 {
                    res += cur_range.1 - cur_range.0;
                } else {
                    res += cur_range.1 - cur_range.0 + 1;
                }
            }
        }
        println!("{:?} {:?} {:?}", last_range, cur_range, res);
        if cur_range.1 > las_top {
            las_top = cur_range.1;
        }
        last_range = cur_range;
    }
    println!("{:?}", ranges);
    println!("{:?}", res);
}

fn do_task_one(inputs: &mut (Vec<(u64, u64)>, Vec<u64>)) {
    let mut res = 0;
    let mut ings = inputs.1.to_owned();
    for range in &inputs.0 {
        let r = RangeInclusive::new(range.0, range.1);
        ings.retain(|ing| {
            println!("proc {:?}", ing);
            if r.contains(&ing) {
                println!("fresh {:?}", ing);
                res += 1;
                false
            } else {
                true
            }
        });
    }
    println!("{:?}", res);
}

fn main() {
    let mut inputs = read_from_file();
    do_task_two(&mut inputs);
    // println!("{:?}", &inputs);
}
