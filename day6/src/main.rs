use std::collections::HashSet;
use std::io::{self, Cursor};
use std::iter::Enumerate;
use std::num;
use std::ops::RangeInclusive;
use std::{fs::File, io::BufRead};

fn read_from_file() -> (Vec<Vec<String>>, Vec<String>, Vec<usize>) {
    let file = File::open("inputs");
    let result: (Vec<Vec<String>>, Vec<String>, Vec<usize>) = match file {
        Ok(file) => {
            let mut lines: Vec<String> = Vec::new();
            io::BufReader::new(file)
                .lines()
                .for_each(|f| lines.push(f.unwrap()));
            let mut ops: Vec<String> = Vec::new();
            let mut col_sizes: Vec<usize> = Vec::new();
            let mut numbers: Vec<Vec<String>> = Vec::new();
            let mut col_index = 0;

            for (id, line) in lines.iter().enumerate().rev() {
                if line.contains("*") || line.contains("+") {
                    let mut count = 0;
                    line.chars().enumerate().for_each(|(i, f)| {
                        if f.is_whitespace() {
                            count += 1;
                        } else {
                            ops.push(f.to_string());
                            if i != 0 {
                                col_sizes.push(count);
                                count = 0;
                            }
                        }
                    });
                    col_sizes.push(count + 1);
                } else {
                    let mut nums: Vec<String> = Vec::new();

                    let mut chars: Vec<char> = line.chars().collect();
                    for i in 0..col_sizes.len() {
                        let mut size = col_sizes[i];
                        let (num, rest) = chars.split_at_mut(size);
                        println!("{:?} {:?}", num, rest);
                        nums.push(num.iter().collect::<String>());
                        chars = Vec::from(rest);
                        if chars.len() > 0 {
                            chars.remove(0);
                        }
                    }
                    numbers.push(nums);
                }
            }
            (numbers, ops, col_sizes)
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn do_task_two(inputs: (Vec<Vec<String>>, Vec<String>, Vec<usize>)) {
    println!("{:?} {:?} {:?}", inputs.0, inputs.1, inputs.2);
    let mut lines = inputs.0.clone();
    let mut columns = vec![];
    lines.reverse();
    for i in 0..inputs.2.len() {
        let mut colum = vec![];
        for line in &lines {
            colum.push(line[i].clone());
        }
        columns.push(colum);
    }

    println!("{:?}", columns);

    let mut res = 0;

    for (index, col) in columns.iter().enumerate() {
        let l = col.first().unwrap().len();
        let mut new_nums: Vec<Vec<char>> = Vec::new();

        for i in 0..l {
            for num in col {
                if i == 0 {
                    new_nums.push(Vec::new());
                }
                new_nums[i].push(num.chars().nth(i).unwrap());
            }
        }

        //println!("new {:?}", new_nums);

        let mut mid_res: u64 = 0;
        for (i, nn) in new_nums.iter().enumerate() {
            let nu = nn.iter().collect::<String>();
            let nuu = nu.trim();
            println!("{:?} {:?}", nuu, &inputs.1[index]);
            if nuu.is_empty() {
                continue;
            }
            let nc = nuu.parse::<u64>().unwrap();
            if &inputs.1[index] == "*" {
                if mid_res == 0 {
                    mid_res = nc;
                } else {
                    mid_res *= nc;
                }
            } else {
                mid_res += nc;
            }
            //println!("{:?}", mid_res);
        }
        res += mid_res;
    }
    println!("{:?}", res);
}

fn do_task_one(inputs: (Vec<Vec<String>>, Vec<String>, Vec<usize>)) {
    let cols = inputs.2.len();
    let mut res: Vec<u64> = vec![0; cols];
    let ops = inputs.1;
    for (i, n) in inputs.0.iter().enumerate() {
        for j in 0..cols {
            let num = n[j].parse().unwrap();
            println!("{:?} {:?} {:?} {:?}", num, res[j], i, j);
            if ops[j] == "+" {
                res[j] += num;
            } else if ops[j] == "*" {
                if res[j] == 0 {
                    res[j] *= num;
                } else {
                    res[j] = num;
                }
            }
        }
    }
    let sum: u64 = res.iter().sum();
    println!("{:?}", sum);
}

fn main() {
    let mut inputs = read_from_file();
    do_task_two(inputs);
    // println!("{:?}", &inputs);
}
