use crate::utils;

const INPUT_PATH: &str = "input/2/ranges.txt";

pub fn solve_p1() -> u64 {
    let string: String = utils::read_file_to_string(INPUT_PATH);
    let ranges = string.split(',');

    //let mut invalid_ids: Vec<u64> = Vec::new();
    let mut sum: u64 = 0;

    for range in ranges {
        let mut endpoints = range.split('-');
        let a: u64 = endpoints.next().unwrap().parse().unwrap();
        let b: u64 = endpoints.next().unwrap().parse().unwrap();

        for id in a..(b+1) {
            if check_if_invalid_p1(&id.to_string()) {
                //invalid_ids.push(id);
                sum += id;
            }
        }
    }

    sum
}

fn check_if_invalid_p1(id: &str) -> bool {
    if id.len() % 2 == 1 {
        return false
    }
    let halfpoint: usize = id.len()/2;
    let firsthalf: &str = &id[0..halfpoint];
    let secondhalf: &str = &id[halfpoint..];

    return firsthalf == secondhalf;
}


pub fn solve_p2() -> u64 {
    let string: String = utils::read_file_to_string(INPUT_PATH);
    let ranges = string.split(',');

    //let mut invalid_ids: Vec<u64> = Vec::new();
    let mut sum: u64 = 0;

    for range in ranges {
        let mut endpoints = range.split('-');
        let a: u64 = endpoints.next().unwrap().parse().unwrap();
        let b: u64 = endpoints.next().unwrap().parse().unwrap();

        for id in a..(b+1) {
            if check_if_invalid_p2(&id.to_string()) {
                //invalid_ids.push(id);
                sum += id;
            }
        }
    }

    sum
}

fn check_if_invalid_p2(id: &str) -> bool {
    let halfpoint: usize = id.len()/2;
    for i in 0..(halfpoint+1) {
        let pattern = &id[0..i];
        
        let mut consists_of_repeated_pattern = true;
        for rem in id.split(pattern) {
            if rem != "" {
                consists_of_repeated_pattern = false;
                break;
            }
        }
        if consists_of_repeated_pattern {
            return true;
        }
    }

    false
}