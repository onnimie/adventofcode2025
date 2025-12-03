use crate::utils;


const INPUT_PATH: &str = "input/3/batteries.txt";

pub fn solve_p1() -> u64 {

    let lines: Vec<String> = utils::read_filelines_to_vec(INPUT_PATH);
    let mut output_joltage: u64 = 0;

    for line in lines {
        let (findx, fval): (usize, u32) = find_first_largest_index_and_number(&line, 0, line.len()-1);
        let (_sindx, sval): (usize, u32) = find_first_largest_index_and_number(&line, findx+1, line.len());

        let joltage: u64 = (fval * 10 + sval) as u64;
        output_joltage += joltage;
    }

    output_joltage
}

fn find_first_largest_index_and_number(string: &String, start_incl: usize, end_excl: usize) -> (usize, u32) {
    let mut max: u32 = 0;
    let mut indx: usize = 0;
    for (i, char) in string.char_indices() {
        if i >= start_incl && i < end_excl {
            let d: u32 = char.to_digit(10).unwrap();
            if d > max {
                max = d;
                indx = i;
            }
        }
    }

    (indx, max)
}



pub fn solve_p2() -> u64 {

    let lines: Vec<String> = utils::read_filelines_to_vec(INPUT_PATH);
    let mut output_joltage: u64 = 0;

    for line in lines {

        let mut joltage: u64 = 0;
        let mut last_val_indx: usize = 0;

        for di in 0..12 {
            let factor: u64 = (10_u64).pow(11-di);

            let (indx, val): (usize, u32) = find_first_largest_index_and_number(&line, last_val_indx, line.len()-(11-(di as usize)));
            last_val_indx = indx+1;

            joltage += (val as u64) * factor;
        }
        //println!("{joltage}");
        output_joltage += joltage;
    }

    output_joltage
}