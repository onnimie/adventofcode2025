use crate::utils;

const INPUT_PATH: &str = "input/5/ids.txt";


pub fn solve_p1() -> u64 {

    let lines: Vec<String> = utils::read_filelines_to_vec(INPUT_PATH);
    let mut iter = lines.split(|s| s == "");

    let id_ranges: &[String] = iter.next().unwrap();
    let ids: &[String] = iter.next().unwrap();

    fn count_fresh_ids(ids: &[String], id_ranges: &[String], start_incl: usize, end_excl: usize, counted: u64) -> u64 {
        
        if start_incl >= end_excl {
            return counted;
        } else {
            let id: u64 = ids[start_incl].parse().unwrap();
            if check_if_spoiled(id_ranges, id) {
                return count_fresh_ids(ids, id_ranges, start_incl+1, end_excl, counted)
            } else {
                return count_fresh_ids(ids, id_ranges, start_incl+1, end_excl, counted+1)
            }
        }
    }

    let count = count_fresh_ids(ids, id_ranges, 0, ids.len(), 0);

    count
}

fn check_if_spoiled(ranges: &[String], id: u64) -> bool {
    for range in ranges {
        let mut split = range.split('-');
        let a: u64 = split.next().unwrap().parse().unwrap();
        let b: u64 = split.next().unwrap().parse().unwrap();

        if id >= a && id <= b {
            return false;
        }
    }
    true
}


pub fn solve_p2() -> u64 {

    fn get_distinct_id_ranges(mut id_ranges: Vec<String>) -> Vec<(u64, u64)> {
        if id_ranges.is_empty() {
            return Vec::new();
        } else {
            let string: String = id_ranges.swap_remove(0);
            let mut range = string.split('-');
            let mut a: u64 = range.next().unwrap().parse().unwrap();
            let mut b: u64 = range.next().unwrap().parse().unwrap();
            let mut nof_removed: usize = 0_usize;

            for (j, other_ran) in id_ranges.clone().iter().enumerate() {
                let mut iter = other_ran.split('-');
                let s: u64 = iter.next().unwrap().parse().unwrap();
                let e: u64 = iter.next().unwrap().parse().unwrap();
                if a >= s {
                    if a <= e {
                        if b > e {
                            a = e + 1;
                        } else {//if b <= e {
                            return get_distinct_id_ranges(id_ranges);
                        }
                    } else { // a > e
                        // do nothing
                    }
                } else { // a < s
                    if b <= e {
                        if b >= s {
                            b = s - 1;
                        } else {//if b < s {
                            // do nothing
                        }
                    } else { // b > e
                        id_ranges.remove(j-nof_removed);
                        nof_removed += 1;
                    }
                }
            }

            let mut ret: Vec<(u64, u64)> = vec![(a,b)];
            ret.append(&mut get_distinct_id_ranges(id_ranges));
            return ret;
        }
    }

    let lines: Vec<String> = utils::read_filelines_to_vec(INPUT_PATH);
    let mut iter = lines.split(|s| s == "");

    let id_ranges: &[String] = iter.next().unwrap();

    let mut count: u64 = 0;
    let v: Vec<(u64, u64)> = get_distinct_id_ranges(id_ranges.to_vec());
    //println!("{:?}", v);
    for (a, b) in v {
        count += b-a + 1;
    }

    count
}