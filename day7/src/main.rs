use std::collections::HashSet;
use std::io::{self, Cursor};
use std::ops::RangeInclusive;
use std::{fs::File, io::BufRead};

fn read_from_file() -> (Vec<(usize, usize)>, (usize, usize), usize) {
    let file = File::open("inputs");
    let result: (Vec<(usize, usize)>, (usize, usize), usize) = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut start: (usize, usize) = (0, 0);
            let mut splits: Vec<(usize, usize)> = Vec::new();
            let mut len = 0;
            for (id, line) in lines.enumerate() {
                if let Ok(line) = line {
                    line.chars().enumerate().for_each(|(i, a)| {
                        if a == 'S' {
                            start = (id, i);
                        }
                        if a == '^' {
                            splits.push((id, i));
                        }
                    });
                }
                len = id;
            }
            (splits, start, len)
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn do_step(
    node: (usize, usize),
    splits: &mut Vec<(usize, usize)>,
    depth: usize,
    seen: &mut HashSet<(usize, usize)>,
    queue: &mut Vec<(usize, usize)>,
    ways: &mut usize,
) {
}

fn do_task_two(inputs: &mut (Vec<(usize, usize)>, (usize, usize), usize)) {
    let mut ways = 1;
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let node = inputs.1;
    let splits: Vec<(usize, usize)> = inputs.0.clone();
    let depth = inputs.2;

    queue.push(node);
    seen.insert(node);

    while !queue.is_empty() {
        if let Some(n) = queue.pop() {
            //println!("check {:?}", n);

            if n.0 == depth {
                // end
                ways += 1;
                println!("{:?}", ways);
            } else {
                if splits.contains(&n) {
                    if !seen.contains(&(n.0, n.1 + 1)) {
                        queue.push((n.0, n.1 + 1));
                    }
                    if !seen.contains(&(n.0, n.1 - 1)) {
                        queue.push((n.0, n.1 - 1));
                    }
                } else {
                    if !seen.contains(&(n.0 + 1, n.1)) {
                        queue.push((n.0 + 1, n.1));
                    }
                }
            }
        }
    }
}

fn do_task_one(inputs: &mut (Vec<(usize, usize)>, (usize, usize), usize)) {
    //println!("{:?} {:?}", inputs.0, inputs.1);
    let mut res = 0;
    let mut beams: HashSet<(usize, usize)> = HashSet::new();
    let splits = inputs.0.clone();
    let start = inputs.1;
    beams.insert(start);
    for i in 1..inputs.2 {
        let mut check_beams: HashSet<(usize, usize)> = HashSet::new();
        beams.iter().for_each(|b| {
            check_beams.insert((b.0 + 1, b.1).clone());
        });

        println!("check beams {:?}", &check_beams);
        let mut new_beams: HashSet<(usize, usize)> = HashSet::new();
        for beam in &check_beams {
            if splits.contains(&beam) {
                res += 1;
                new_beams.insert((beam.0, beam.1 + 1).clone());
                new_beams.insert((beam.0, beam.1 - 1).clone());
                println!("hit {:?}", beam);
            } else {
                new_beams.insert(beam.clone());
            }
        }
        if !new_beams.is_empty() {
            println!("new beams {:?}", &new_beams);
            beams = new_beams;
        } else {
            beams = check_beams.clone();
        }
    }
    println!("{:?}", res);
}

fn main() {
    let mut inputs = read_from_file();
    do_task_two(&mut inputs);
    // println!("{:?}", &inputs);
}
