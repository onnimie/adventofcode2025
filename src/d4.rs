use crate::utils;

const INPUT_PATH: &str = "input/4/rolls.txt";


pub fn solve_p1() -> u64 {

    let lines: Vec<String> = utils::read_filelines_to_vec(INPUT_PATH);
    let mut nof_accessible = 0_u64;

    for row in 0..lines.len() {
        for col in 0..lines[0].chars().count() {

            if &(lines[row])[col..col+1] == "@" {

                let count = count_adjacent_paper_rolls(row, col, &lines);

                if count < 4 {
                    nof_accessible += 1;
                }
            }
        }
    }

    nof_accessible
}

fn count_adjacent_paper_rolls(row: usize, col: usize, lines: &Vec<String>) -> u64 {

    let mut count: u64 = 0;
    let max_col: usize = lines[0].chars().count()-1;
    let max_row: usize = lines.len()-1;

    if row > 0 {
        let up_neighbor: &str = &(lines[row-1])[col..col+1];
        if up_neighbor == "@" {
            count += 1;
        }
    }
    if row < max_row {
        let down_neighbor: &str = &(lines[row+1])[col..col+1];
        if down_neighbor == "@" {
            count += 1;
        }
    }
    if col > 0 {
        let left_neighbor: &str = &(lines[row])[col-1..col];
        if left_neighbor == "@" {
            count += 1;
        }
    }
    if col < max_col {
        let right_neighbor: &str = &(lines[row])[col+1..col+2];
        if right_neighbor == "@" {
            count += 1;
        }
    }

    if col > 0 && row > 0 {
        let up_left_neighbor: &str = &(lines[row-1])[col-1..col];
        if up_left_neighbor == "@" {
            count += 1;
        }
    }
    if col < max_col && row < max_row {
        let down_right_neighbor: &str = &(lines[row+1])[col+1..col+2];
        if down_right_neighbor == "@" {
            count += 1;
        }
    }
    if col < max_col && row > 0 {
        let up_right_neighbor: &str = &(lines[row-1])[col+1..col+2];
        if up_right_neighbor == "@" {
            count += 1;
        }
    }
    if col > 0 && row < max_row {
        let down_left_neighbor: &str = &(lines[row+1])[col-1..col];
        if down_left_neighbor == "@" {
            count += 1;
        }
    }

    count
}


fn remove_accessible_rolls(lines: &mut Vec<String>) -> u64 {

    let mut nof_accessible = 0_u64;

    for row in 0..lines.len() {
        for col in 0..lines[0].chars().count() {

            if &(lines[row])[col..col+1] == "@" {

                let count = count_adjacent_paper_rolls(row, col, &lines);

                if count < 4 {
                    nof_accessible += 1;

                    lines[row].replace_range(col..col+1, ".");
                }
            }
        }
    }

    nof_accessible
}


pub fn solve_p2() -> u64 {
    let mut lines = utils::read_filelines_to_vec(INPUT_PATH);
    let mut counter: u64 = 0;

    let mut nof_removed: u64 = remove_accessible_rolls(&mut lines);
    while nof_removed > 0 {
        counter += nof_removed;

        nof_removed = remove_accessible_rolls(&mut lines);
    }

    counter
}