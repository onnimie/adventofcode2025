use std::f32;

use crate::utils;

const INPUT_PATH: &str = "input/8/junctions.txt";

pub fn solve_p1_wrong() -> u64 {
    let lines: Vec<String> = utils::read_filelines_to_vec(INPUT_PATH);

    let mut circuits: Vec<Vec<usize>> = Vec::new();
    for i in 0..lines.len() {
        circuits.push(vec![i]);
    }
    
    let mut mindistances: Vec<(f32, usize, usize)> = Vec::new();

    for i in 0..lines.len() {
        let (d, j) = find_closest_box(i, &lines, &circuits);
        mindistances.push((d, i, j));
    }

    mindistances.sort_by(|a: &(f32, usize, usize), b: &(f32, usize, usize)| a.0.total_cmp(&b.0));

    for (_d, i, j) in mindistances {

        let mut i_circ: usize = circuits.len();
        let mut j_circ: usize = circuits.len();
        for (k, c) in circuits.iter().enumerate() {
            if c.contains(&i) {
                i_circ = k;
            }
            if c.contains(&j) {
                j_circ = k;
            }
        }

        if !(i_circ == j_circ) {
            let mut clone: Vec<Vec<usize>> = circuits.clone();
            clone[i_circ].append(&mut circuits[j_circ]);
            clone.swap_remove(j_circ);
            circuits = clone;
        }
    }
    /*
    let mut _wires_placed: u64 = 0;
    let mut _iterations: u64 = 0;

    loop {
        let mut all_are_connected: bool = true;
        let mut mindistances: Vec<(f32, usize, usize)> = Vec::new();

        for i in 0..lines.len() {
            let (d, j) = find_closest_nonconnected_box(i, &lines, &circuits);

            if j != i {
                all_are_connected = false;
                mindistances.push((d, i, j));
            }
        }

        if all_are_connected {
            break;
        } else {
            mindistances.sort_by(|a: &(f32, usize, usize), b: &(f32, usize, usize)| a.0.total_cmp(&b.0));
            let (_d, i, j): (f32, usize, usize) = mindistances[0];

            let mut i_circ: usize = 0;
            let mut j_circ: usize = 0;
            for (k, c) in circuits.iter().enumerate() {
                if c.contains(&i) {
                    i_circ = k;
                }
                if c.contains(&j) {
                    j_circ = k;
                }
            }

            if !(i_circ == j_circ) {
                let mut clone: Vec<Vec<usize>> = circuits.clone();
                clone[i_circ].append(&mut circuits[j_circ]);
                clone.swap_remove(j_circ);
                circuits = clone;

                _wires_placed += 1;
            } else {
                assert!(false);
            }

        }

        //dbg!(&circuits);
        //let _ = std::io::stdin().read_line(&mut String::new());

        _iterations += 1;
        //if iterations >= 10 {
        //    break;
        //}
        println!("{_iterations}");
    }*/

    circuits.sort_by(|a: &Vec<usize>, b: &Vec<usize>| b.len().cmp(&a.len()));
    let t: Vec<u64> = circuits.iter().map(|v: &Vec<usize>| v.len() as u64).collect();
    let sum: u64 = t.iter().sum();
    dbg!(&t);
    //dbg!(circuits);
    dbg!(sum);
    t[0] * t[1] * t[2]

}


fn get_coords(s: &str) -> (f32, f32, f32) {
    let mut iter = s.split(',').map(|t| t.parse().unwrap());
    (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
}

fn distance(p1: (f32, f32, f32), p2: (f32, f32, f32)) -> f32 {
    ((p1.0-p2.0)*(p1.0-p2.0) + (p1.1-p2.1)*(p1.1-p2.1) + (p1.2-p2.2)*(p1.2-p2.2)).sqrt()
}

fn find_closest_box(a_indx: usize, all_boxes: &Vec<String>, _circuits: &Vec<Vec<usize>>) -> (f32, usize) {
    let mut current_min: f32 = f32::MAX;
    let mut current_pair: usize = a_indx;

    let p1: (f32, f32, f32) = get_coords(&all_boxes[a_indx]);
    for j in 0..all_boxes.len() {
        if a_indx != j {//mpy !connected_boxes.contains(&j) {
            let p2: (f32, f32, f32) = get_coords(&all_boxes[j]);
            let d: f32 = distance(p1, p2);

            if d < current_min {
                current_min = d;
                current_pair = j;
            }
        }
    }
    assert!(current_pair != a_indx);
    (current_min, current_pair)
}

fn find_closest_nonconnected_box(a_indx: usize, all_boxes: &Vec<String>, circuits: &Vec<Vec<usize>>) -> (f32, usize) {
    let mut current_min: f32 = f32::MAX;
    let mut current_pair: usize = a_indx;

    let mut connected_boxes: Vec<usize> = vec![a_indx];
    for c in circuits {
        if c.contains(&a_indx) {
            connected_boxes = c.clone();
        }
    }

    let p1: (f32, f32, f32) = get_coords(&all_boxes[a_indx]);
    for j in 0..all_boxes.len() {
        if !connected_boxes.contains(&j) {
            let p2: (f32, f32, f32) = get_coords(&all_boxes[j]);
            let d: f32 = distance(p1, p2);

            if d < current_min {
                current_min = d;
                current_pair = j;
            }
        }
    }

    (current_min, current_pair)
}


pub fn solve_p1() -> u64 {
    let lines: Vec<String> = utils::read_filelines_to_vec(INPUT_PATH);

    let mut circuits: Vec<Vec<usize>> = Vec::new();
    for i in 0..lines.len() {
        circuits.push(vec![i]);
    }

    let mut pair_distances: Vec<(f32, usize, usize)> = Vec::new();

    for i in 0..(lines.len()-1) {
        let p1: (f32, f32, f32) = get_coords(&lines[i]);
        for j in i+1..lines.len() {
            let p2: (f32, f32, f32) = get_coords(&lines[j]);
            let d: f32 = distance(p1, p2);
            pair_distances.push((d, i, j));
        }
    }

    pair_distances.sort_by(|a: &(f32, usize, usize), b: &(f32, usize, usize)| a.0.total_cmp(&b.0));

    let mut last_connected_boxes_mult: u64 = 0;

    for (_d, i, j) in pair_distances {

        let mut i_circ: usize = circuits.len();
        let mut j_circ: usize = circuits.len();
        for (k, c) in circuits.iter().enumerate() {
            if c.contains(&i) {
                i_circ = k;
            }
            if c.contains(&j) {
                j_circ = k;
            }
        }

        if !(i_circ == j_circ) {
            let mut clone: Vec<Vec<usize>> = circuits.clone();
            clone[i_circ].append(&mut circuits[j_circ]);
            clone.swap_remove(j_circ);
            circuits = clone;
        }

        if circuits.len() == 1 {
            let ix: u64 = lines[i].split(',').next().unwrap().parse().unwrap();
            let jx: u64 = lines[j].split(',').next().unwrap().parse().unwrap();
            last_connected_boxes_mult = ix * jx;
            break;
        }
    }

    //circuits.sort_by(|a: &Vec<usize>, b: &Vec<usize>| b.len().cmp(&a.len()));
    //let t: Vec<u64> = circuits.iter().map(|v: &Vec<usize>| v.len() as u64).collect();
    //let sum: u64 = t.iter().sum();
    //dbg!(&t);
    //dbg!(circuits);
    //dbg!(sum);
    //t[0] * t[1] * t[2]
    last_connected_boxes_mult
}