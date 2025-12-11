extern crate itertools;

use std::{
    fs::File,
    io::{self, BufRead},
};

use itertools::Itertools;

fn read_from_file() -> (Vec<(usize, usize, usize)>) {
    let file = File::open("inputs");
    let result: (Vec<(usize, usize, usize)>) = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut boxes: Vec<(usize, usize, usize)> = Vec::new();
            for (id, line) in lines.enumerate() {
                if let Ok(line) = line {
                    let coord_split: Vec<&str> = line.split(',').collect();
                    let jbox: (usize, usize, usize) = (
                        coord_split[0].parse().unwrap(),
                        coord_split[1].parse().unwrap(),
                        coord_split[2].parse().unwrap(),
                    );
                    boxes.push(jbox);
                }
            }
            (boxes)
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn union(
    x: usize,
    y: usize,
    parent: &mut Vec<usize>,
    ranks: &mut Vec<usize>,
    size: &mut Vec<usize>,
) -> usize {
    let mut rx = find(x, parent, ranks, size);
    let mut ry = find(y, parent, ranks, size);

    if rx == ry {
        return 0;
    }
    if size[rx] < size[ry] {
        (rx, ry) = (ry, rx);
    }
    parent[ry] = rx;
    size[rx] += size[ry];
    size[ry] = 0;
    return size[rx];
}

fn find(x: usize, parent: &Vec<usize>, ranks: &mut Vec<usize>, size: &mut Vec<usize>) -> usize {
    let mut y = x;
    while parent[y] != y {
        y = parent[y];
        //ranks[y] = 0;
    }
    return y;
}

fn do_task_one(inputs: &mut (Vec<(usize, usize, usize)>)) {
    let mut boxes: Vec<(usize, usize, usize)> = inputs.clone();

    let mut pairs = (0..boxes.len()).tuple_combinations().collect::<Vec<_>>();
    pairs.sort_by_key(|&(i, j)| {
        let (x1, y1, z1) = boxes[i];
        let (x2, y2, z2) = boxes[j];
        ((x2 as i64 - x1 as i64).pow(2)
            + (y2 as i64 - y1 as i64).pow(2)
            + (z2 as i64 - z1 as i64).pow(2))
    });

    let mut parent: Vec<usize> = vec![0; boxes.len()];
    (0..boxes.len()).into_iter().for_each(|x| parent[x] = x);
    let mut ranks: Vec<usize> = vec![0; boxes.len()];
    let mut size: Vec<usize> = vec![1; boxes.len()];

    let mut e = 0;
    let mut i = 0;

    while e < boxes.len() - 1 && i < 1000 {
        let next_edge = pairs[i].clone();
        i += 1;

        union(next_edge.0, next_edge.1, &mut parent, &mut ranks, &mut size);
    }

    let mut prod: usize = 1;
    size.sort();
    for i in 0..3 {
        prod *= size.pop().unwrap();
    }
    println!("res {:?}", prod);
}

fn do_task_two(inputs: &mut (Vec<(usize, usize, usize)>)) {
    let mut boxes: Vec<(usize, usize, usize)> = inputs.clone();

    let mut pairs = (0..boxes.len()).tuple_combinations().collect::<Vec<_>>();
    pairs.sort_by_key(|&(i, j)| {
        let (x1, y1, z1) = boxes[i];
        let (x2, y2, z2) = boxes[j];
        ((x2 as i64 - x1 as i64).pow(2)
            + (y2 as i64 - y1 as i64).pow(2)
            + (z2 as i64 - z1 as i64).pow(2))
    });

    let mut parent: Vec<usize> = vec![0; boxes.len()];
    (0..boxes.len()).into_iter().for_each(|x| parent[x] = x);
    let mut ranks: Vec<usize> = vec![0; boxes.len()];
    let mut size: Vec<usize> = vec![1; boxes.len()];

    let mut i = 0;

    let mut last = 0;
    while i < pairs.len() {
        let next_edge = pairs[i].clone();

        let mut s = union(next_edge.0, next_edge.1, &mut parent, &mut ranks, &mut size);
        if (s == boxes.len()) {
            last = i;
        }
        i += 1;
        //println!("{:?} {:?}", size, s);
    }

    let mut edge = pairs[last];
    //println!("{:?}", edge);
    //println!("{:?} {:?}", boxes[edge.0], boxes[edge.1]);

    println!("res {:?}", boxes[edge.0].0 * boxes[edge.1].0);
}

fn main() {
    let mut inputs = read_from_file();
    //do_task_one(&mut inputs);
    do_task_two(&mut inputs);
    // println!("{:?}", &inputs);
}
