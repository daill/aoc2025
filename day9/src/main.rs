extern crate itertools;

use std::{
    cmp::{max, min},
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

use itertools::Itertools;

fn read_from_file() -> (Vec<(usize, usize)>, usize, usize) {
    let file = File::open("test");
    let result: (Vec<(usize, usize)>, usize, usize) = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut boxes: Vec<(usize, usize)> = Vec::new();
            let mut maxx = 0;
            let mut maxy = 0;
            for (id, line) in lines.enumerate() {
                if let Ok(line) = line {
                    let coord_split: Vec<&str> = line.split(',').collect();
                    let x: usize = coord_split.get(0).unwrap().parse().unwrap();
                    if maxx < x {
                        maxx = x;
                    }
                    let y: usize = coord_split.get(1).unwrap().parse().unwrap();

                    boxes.push((
                        coord_split.get(0).unwrap().parse().unwrap(),
                        coord_split.get(1).unwrap().parse().unwrap(),
                    ));
                }
                maxy = id;
            }
            (boxes, maxx, maxy)
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn do_task_one(inputs: &mut (Vec<(usize, usize)>)) {
    let tiles: Vec<(usize, usize)> = inputs.clone();
    let mut pairs: Vec<(usize, usize)> = (0..tiles.len()).tuple_combinations().collect::<Vec<_>>();
    let mut sizes: Vec<(usize, usize, usize)> = Vec::new();
    for (a, b) in pairs {
        let x = (tiles[b].0 as isize - tiles[a].0 as isize).abs() as usize + 1;
        let y = (tiles[b].1 as isize - tiles[a].1 as isize).abs() as usize + 1;
        sizes.push((a, b, x * y));
    }

    sizes.sort_by(|a, b| a.2.cmp(&b.2));
    //sizes
    //    .iter()
    //    .for_each(|f| println!("{:?} {:?} {:?}", tiles[f.0], tiles[f.1], f.2));
    println!("{:?}", sizes.pop().unwrap().2);
}

fn check_in_polygom(point: (usize, usize), tiles: &Vec<(usize, usize)>) -> bool {
    tiles.push(tiles[0]);
}

fn do_task_two(inputs: &mut (Vec<(usize, usize)>, usize, usize)) {
    let tiles: Vec<(usize, usize)> = inputs.0.clone();
    let mut pairs: Vec<(usize, usize)> = (0..tiles.len()).tuple_windows().collect::<Vec<_>>();

    let mut sizes: Vec<(usize, usize, usize, bool)> = Vec::new();
    for (a, b) in pairs {
        let x = (tiles[b].0 as isize - tiles[a].0 as isize).abs() as usize + 1;
        let y = (tiles[b].1 as isize - tiles[a].1 as isize).abs() as usize + 1;
        sizes.push((a, b, x * y, false));
    }

    let mut triples_counter = (0..tiles.len()).collect::<Vec<usize>>();
    for a in triples_counter.windows(3) {
        let mut e1 = tiles[a[0]];
        let mut e2 = tiles[a[1]];
        let mut e3 = tiles[a[2]];
        
        let left = 
    }

    sizes.sort_by(|a, b| a.2.cmp(&b.2));

    println!("{:?}", sizes);
}

fn main() {
    let mut inputs = read_from_file();
    do_task_two(&mut inputs);
    // println!("{:?}", &inputs);
}
