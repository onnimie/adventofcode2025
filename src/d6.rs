use crate::utils;

const INPUT_PATH: &str = "input/6/problems.txt";
const NOF_ROWS: usize = 4;

pub fn solve_p1() -> u64 {
    let lines = utils::read_filelines_to_vec(INPUT_PATH);
    let mut nums: Vec<Vec<u64>> = Vec::new();
    let mut op_is_add: Vec<bool> = Vec::new();

    for i in 0..NOF_ROWS {
        let mut v: Vec<u64> = Vec::new();
        for ns in lines[i].split(' ').filter(|s| !s.is_empty()) {
            let n: u64 = ns.parse().unwrap();
            v.push(n);
        }
        nums.push(v);
    }

    for sop in lines[NOF_ROWS].split(' ').filter(|s| !s.is_empty()) {
        if sop == "+" {
            op_is_add.push(true);
        } else {
            op_is_add.push(false);
        }
    }

    let nof_problems: usize = nums[0].len();
    let mut grand_total: u64 = 0;
    for p in 0..nof_problems {
        let mut total: u64 = nums[0][p];
        for row in 1..NOF_ROWS {
            let n: u64 = nums[row][p];
            if op_is_add[p] {
                total += n;
            } else {
                total *= n;
            }
        }
        grand_total += total;
    }
    

    grand_total
}


pub fn solve_p2() -> u64 {
    let lines = utils::read_filelines_to_vec(INPUT_PATH);

    let row_len: usize = lines[0].len();
    
    let mut op_iter: std::str::Chars<'_> =  lines[NOF_ROWS].chars();
    let mut num_iters: Vec<std::str::Chars<'_>> = Vec::new();
    for i in 0..NOF_ROWS {
        num_iters.push(lines[i].chars());
    }

    let mut i: usize = 0;
    let mut grand_total: u64 = 0;
    let mut sub_total: u64 = 0;
    let mut current_op_is_add: bool = true;
    while i < row_len {
        let op = op_iter.next().unwrap();
        
        let mut nums: Vec<char> = Vec::new();
        for niter in &mut num_iters {
            nums.push(niter.next().unwrap());
        }

        if op == '+' || op == '*' {
            grand_total += sub_total;
            current_op_is_add = op == '+';
            if current_op_is_add {
                sub_total = 0;
            } else {
                sub_total = 1;
            }
        }

        let mut ns: String = String::from("");
        for j in nums {
            if j == ' ' {
                continue;
            } else {
                //let d: u64 = j.to_digit(10).unwrap() as u64;
                //n += d * 10_u64.pow((NOF_ROWS-k) as u32);

                ns.insert(ns.len(), j);
            }
        }
        //println!("{ns}");
        if !ns.is_empty() {
            let n: u64 = ns.parse().unwrap();
            if current_op_is_add {
                sub_total += n;
            } else {
                sub_total *= n;
            }
        }

        

        i += 1;
    }
    grand_total += sub_total;

    grand_total
}