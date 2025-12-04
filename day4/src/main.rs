use std::collections::HashSet;
use std::io::{self, Write};
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

fn process(inputs: &HashSet<(isize, isize)>) -> HashSet<(isize, isize)> {
    let dir = [
        (0, 1),
        (1, 0),
        (1, 1),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    let mut result: HashSet<(isize, isize)> = HashSet::new();
    let mut i = 0;
    for roll in inputs {
        i += 1;
        //println!("{:?}", roll);
        let mut n = 0;
        for d in dir {
            if inputs.contains(&(roll.0 + d.0, roll.1 + d.1)) {
                n += 1;
            }
            //println!("check {:?} {:?} {:?}", &(roll.0 + d.0, roll.1 + d.1), n, i);
        }
        if n < 4 {
            result.insert(roll.clone());
        }
    }
    return result;
}

fn do_task_one(inputs: &HashSet<(isize, isize)>) {
    let mut result = process(&inputs);
    println!("{:?}", result.len());
}

fn do_task_two(inputs: &HashSet<(isize, isize)>) {
    let mut workset = inputs.clone();
    let mut r = 0;
    let mut result = process(&workset);

    println!("{:?}", workset);
    while result.len() > 0 {
        r += result.len();
        for i in result {
            workset.remove(&i);
        }
        println!("{:?}", workset);
        result = process(&workset);
    }
    println!("{:?}", r);
}

fn main() {
    let inputs = read_from_file();
    do_task_two(&inputs);
    // println!("{:?}", &inputs);
}
