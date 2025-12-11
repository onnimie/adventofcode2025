use std::sync::Arc;

use crate::utils;

const INPUT_PATH: &str = "input/7/manifold.txt";

pub fn solve_p1() -> u64 {
    let mut lines: Vec<Vec<char>> = utils::read_filelines_to_vec_of_char(INPUT_PATH);
    let nof_rows: usize = lines.len();

    let mut count: u64 = 0;

    for line_indx in 1..nof_rows-1 {
        for (i, c) in lines[line_indx].clone().iter().enumerate() {
            match *c {
                '.' => {
                    match lines[line_indx-1][i] {
                        'S' => lines[line_indx][i] = '|',
                        '|' => lines[line_indx][i] = '|',
                        _ => {} //,
                    }
                },
                '|' => {},
                '^' => {
                    match lines[line_indx-1][i] {
                        '|' => {
                            count += 1;
                            lines[line_indx][i-1] = '|';
                            lines[line_indx][i+1] = '|';
                        },
                        _ => {} //,
                    }
                },
                _ => {},
            }
        }
    }

    count
}


pub fn solve_p2_slowww() -> u64 {
    let available_threads: usize = std::thread::available_parallelism().unwrap().get();
    let lines: Arc<Vec<Vec<char>>> = Arc::new(utils::read_filelines_to_vec_of_char(INPUT_PATH));

    fn rekursio(start_row: usize, start_col: usize, lines: Arc<Vec<Vec<char>>>, available_threads: usize) -> u64 {
        let nof_rows: usize = lines.len();
        
        if start_row >= nof_rows {
            return 1;
        }

        let current_char: char = lines[start_row][start_col];

        match current_char {
            '.' => {
                rekursio(start_row+1, start_col, lines, available_threads)
            },
            //'|' => {}, //shouldnt be possible
            '^' => {
                if available_threads >= 2 {

                    let l1: Arc<Vec<Vec<char>>> = Arc::clone(&lines);
                    let l2: Arc<Vec<Vec<char>>> = Arc::clone(&lines);

                    let t1: std::thread::JoinHandle<u64> = std::thread::spawn(move || {
                        let ret = rekursio(start_row+2, start_col-1, l1, available_threads/2);
                        ret
                    });

                    let t2: std::thread::JoinHandle<u64> = std::thread::spawn(move || {
                        let ret = rekursio(start_row+2, start_col+1, l2, available_threads/2);
                        ret
                    });

                    let res: u64 = t1.join().unwrap() + t2.join().unwrap();
                    res

                } else {
                    rekursio(start_row+2, start_col-1, Arc::clone(&lines), available_threads) + rekursio(start_row+2, start_col+1, lines, available_threads)
                }
                
            },
            _ => {0},
        }
    }

    let (start_col, _k) = lines[0].iter().enumerate().find(|c| *c.1=='S').unwrap();
    
    rekursio(1, start_col, Arc::clone(&lines), available_threads)
}


fn get_missing_binary_tree_nodes(lines: &Vec<Vec<char>>, start_row: usize, start_col: usize, row_step_size: usize) -> Vec<Vec<bool>> {
    let mut row_i: usize = start_row;
    let mut ret: Vec<Vec<bool>> = Vec::new();
    let mut current_cols: Vec<usize> = vec![start_col];
    while row_i < lines.len() {
        let mut level: Vec<bool> = Vec::new();
        let mut new_cols: Vec<usize> = Vec::new();

        for col in &current_cols {
            if lines[row_i][*col] == '^' {
                level.push(true);
            } else {
                level.push(false);
            }
            new_cols.push(*col - 1);
        }
        new_cols.push(current_cols.last().unwrap() + 1);

        row_i += row_step_size;
        current_cols = new_cols;
        ret.push(level);
    }

    ret
}


pub fn solve_p2() -> u64 {
    let lines: Vec<Vec<char>> = utils::read_filelines_to_vec_of_char(INPUT_PATH);
    let (start_col, _k) = lines[0].iter().enumerate().find(|c| *c.1=='S').unwrap();

    let btree: Vec<Vec<bool>> = get_missing_binary_tree_nodes(&lines, 2, start_col, 2);
    let mut nof_timelines_going_into_splitter: Vec<Vec<u64>> = Vec::new();
    for i in 0..btree.len()+1 {
        nof_timelines_going_into_splitter.push(vec![0; i+1])
    }
    nof_timelines_going_into_splitter[0][0] = 1;

    for level in 0..btree.len() {
        for splitter in 0..btree[level].len() {
            
            let t: u64 = nof_timelines_going_into_splitter[level][splitter];
            if btree[level][splitter] {
                nof_timelines_going_into_splitter[level+1][splitter] += t;
                nof_timelines_going_into_splitter[level+1][splitter+1] += t;
            } else {
                if level != btree.len()-1 {
                    nof_timelines_going_into_splitter[level+2][splitter+1] += t;
                } else {
                    nof_timelines_going_into_splitter[level+1][splitter+1] += t;
                }
                
            }
        }
    }

    nof_timelines_going_into_splitter.last().unwrap().iter().sum()
}