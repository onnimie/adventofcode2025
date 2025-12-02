use crate::utils;

const INIT: i32 = 50;
const INPUT_PATH: &str = "input/1/rotations.txt";

pub fn solve_p1() -> u64 {

    let lines: Vec<String> = utils::read_filelines_to_vec(INPUT_PATH);

    let mut current_val: i32 = INIT;
    let mut counter: u64 = 0;

    for line in lines {
        let dir = line.chars().next().unwrap();
        let dir_factor: i32 = match dir {
            'R' => 1,
            _ => -1,
        };

        let num_slice = &line[1..];
        let rotations: i32 = num_slice.parse().unwrap();

        current_val = ((current_val + ((rotations % 100) * dir_factor)) + 200) % 100;
        if current_val == 0 {
            counter += 1;
        } 
    }

    counter
}

pub fn solve_p2() -> u64 {
    let lines: Vec<String> = utils::read_filelines_to_vec(INPUT_PATH);

    let mut current_val: i32 = INIT;
    let mut counter: u64 = 0;

    for line in lines {
        let dir = line.chars().next().unwrap();
        let dir_factor: i32 = match dir {
            'R' => 1,
            _ => -1,
        };

        let num_slice = &line[1..];
        let mut rotations: i32 = num_slice.parse().unwrap();

        while rotations >= 100 {
            counter += 1;
            rotations -= 100;
        }

        match dir_factor {
            1 => if current_val + rotations > 99 { counter += 1 },
            _ => if current_val - rotations <= 0 && current_val != 0 { counter += 1 },
        }

        current_val = ((current_val + ((rotations % 100) * dir_factor)) + 200) % 100;
        //if current_val == 0 {
        //    counter += 1;
        //} 
    }

    counter
}