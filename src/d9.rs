use crate::utils;

const INPUT_PATH: &str = "input/9/tiles.txt";

pub fn solve_p1() -> u64 {
    let lines: Vec<String> = utils::read_filelines_to_vec(INPUT_PATH);

    let mut biggest_area: u64 = 0;

    for i in 0..(lines.len()-1) {
        let mut si = lines[i].split(',');
        let p1: (i64, i64) = (si.next().unwrap().parse().unwrap(), si.next().unwrap().parse().unwrap());

        for j in i+1..lines.len() {
            let mut sj = lines[j].split(',');
            let p2: (i64, i64) = (sj.next().unwrap().parse().unwrap(), sj.next().unwrap().parse().unwrap());

            let area = get_area_from_corners(p1, p2);
            if area > biggest_area {
                biggest_area = area;
            }
        }
    }

    biggest_area
}

fn get_area_from_corners(p1: (i64, i64), p2: (i64, i64)) -> u64 {
    let a: i64 = (p1.0-p2.0).abs()+1;
    let b: i64 = (p1.1-p2.1).abs()+1;
    (a * b) as u64
}

fn get_pos_from_line(line: &str) -> (u64, u64) {
    let mut iter = line.split(',');
    (iter.next().unwrap().parse().unwrap(), iter.next().unwrap().parse().unwrap())
}

pub fn solve_p2_shit() -> u64 {
    let mut lines: Vec<String> = utils::read_filelines_to_vec(INPUT_PATH);
    lines.push(lines[0].clone());

    let (right_turns, left_turns) = get_nof_right_and_left_turns(&lines);

    let edge_positions: Vec<(u64, u64)> = get_edge_positions(&lines);
    dbg!(edge_positions.len());

    dbg!(right_turns);
    dbg!(left_turns);

    lines.swap_remove(lines.len()-1);

    let mut biggest_area: u64 = 0;

    for i in 0..(lines.len()-1) {
        let p1: (u64, u64) = get_pos_from_line(&lines[i]);
        for j in (i+1)..lines.len() {
            let p2: (u64, u64) = get_pos_from_line(&lines[j]);

            let mut area_is_valid: bool = true;
            if p1.0 == p2.0 {
                let x: u64 = p1.0;
                let mut has_edge: bool = false;
                let mut has_dot: bool = false;
                if p1.1 > p2.1 {
                    for y in (p2.1+1)..p1.1 {
                        if edge_positions.contains(&(x,y)) {
                            has_edge = true;
                        } else {
                            has_dot = true;
                        }
                        if has_edge && has_dot {
                            area_is_valid = false;
                            break;
                        }
                    }
                } else {
                    for y in (p1.1+1)..p2.1 {
                        if edge_positions.contains(&(x,y)) {
                            has_edge = true;
                        } else {
                            has_dot = true;
                        }
                        if has_edge && has_dot {
                            area_is_valid = false;
                            break;
                        }
                    }
                }
            } else if p1.1 == p2.1 {
                let y: u64 = p1.1;
                let mut has_edge: bool = false;
                let mut has_dot: bool = false;
                if p1.0 > p2.0 {
                    for x in (p2.0+1)..p1.0 {
                        if edge_positions.contains(&(x,y)) {
                            has_edge = true;
                        } else {
                            has_dot = true;
                        }
                        if has_edge && has_dot {
                            area_is_valid = false;
                            break;
                        }
                    }
                } else {
                    for x in (p1.0+1)..p2.0 {
                        if edge_positions.contains(&(x,y)) {
                            has_edge = true;
                        } else {
                            has_dot = true;
                        }
                        if has_edge && has_dot {
                            area_is_valid = false;
                            break;
                        }
                    }
                }
            } else {
                println!("starting,,");
                let mut xpositions: Vec<u64> = Vec::new();
                if p1.0 > p2.0 {
                    for x in p2.0+1..p1.0 {
                        xpositions.push(x);
                    }
                } else {
                    for x in p1.0+1..p2.0 {
                        xpositions.push(x);
                    }
                }
                println!("x done");
                let mut ypositions: Vec<u64> = Vec::new();
                if p1.1 > p2.1 {
                    for y in p2.1+1..p1.1 {
                        ypositions.push(y);
                    }
                } else {
                    for y in p1.1+1..p2.1 {
                        ypositions.push(y);
                    }
                }
                println!("y done");
                let mut has_edge: bool = false;
                let mut has_dot: bool = false;
                for x in xpositions {
                    for y in &ypositions {
                        if edge_positions.contains(&(x,*y)) {
                            has_edge = true;
                        } else {
                            has_dot = true;
                        }
                        if has_edge && has_dot {
                            area_is_valid = false;
                            break;
                        }
                    }
                }
                println!("done");
            }

            if area_is_valid {
                let area = get_area_from_corners((p1.0 as i64, p1.1 as i64), (p2.0 as i64, p2.1 as i64));
                if area > biggest_area {
                    biggest_area = area;
                }
            }

            println!("{i}, {j}");
            
        }
    } 


    biggest_area
}

fn get_edge_positions(lines: &Vec<String>) -> Vec<(u64, u64)> {
    let mut prev_pos: (u64, u64) = get_pos_from_line(&lines[0]);
    let mut current_pos: (u64, u64);
    let mut edge_positions: Vec<(u64, u64)> = Vec::new();

    for i in 1..lines.len() {
        current_pos = get_pos_from_line(&lines[i]);

        if current_pos.0 == prev_pos.0 {
            if current_pos.1 > prev_pos.1 {
                for k in (prev_pos.1+1)..(current_pos.1) {
                    edge_positions.push((prev_pos.0, k));
                }
            } else {
                for k in (current_pos.1+1)..(prev_pos.1) {
                    edge_positions.push((prev_pos.0, k));
                }
            }
        } else {
            if current_pos.0 > prev_pos.0 {
                for k in (prev_pos.0+1)..(current_pos.0) {
                    edge_positions.push((k, prev_pos.1));
                }
            } else {
                for k in (current_pos.0+1)..(prev_pos.0) {
                    edge_positions.push((k, prev_pos.1));
                }
            }
        }
        prev_pos = current_pos;
    }

    edge_positions
}

fn get_offedge_positions(lines: &Vec<String>, right: bool) -> Vec<(u64, u64)> {
    let mut prev_pos: (u64, u64) = get_pos_from_line(&lines[0]);
    let mut current_pos: (u64, u64);
    let mut edge_positions: Vec<(u64, u64)> = Vec::new();

    for i in 1..lines.len() {
        current_pos = get_pos_from_line(&lines[i]);

        if current_pos.0 == prev_pos.0 {
            if current_pos.1 > prev_pos.1 {
                //dir = (0, 1);
                for k in (prev_pos.1+1)..(current_pos.1) {
                    if right { edge_positions.push((prev_pos.0+1, k)); }
                    else { edge_positions.push((prev_pos.0-1, k)); }
                }
            } else {
                //dir = (0, -1);
                for k in (current_pos.1+1)..(prev_pos.1) {
                    if right { edge_positions.push((prev_pos.0-1, k)); }
                    else { edge_positions.push((prev_pos.0+1, k)); }
                }
            }
        } else {
            if current_pos.0 > prev_pos.0 {
                //dir = (1, 0);
                for k in (prev_pos.0+1)..(current_pos.0) {
                    if right { edge_positions.push((k, prev_pos.1-1)); }
                    else { edge_positions.push((k, prev_pos.1+1)); }
                }
            } else {
                //dir = (-1, 0);
                for k in (current_pos.0+1)..(prev_pos.0) {
                    if right { edge_positions.push((k, prev_pos.1+1)); }
                    else { edge_positions.push((k, prev_pos.1-1)); }
                }
            }
        }
        prev_pos = current_pos;
    }
    /*
    println!("removing overwritten offedges..");
    let mut bad_edges: Vec<usize> = Vec::new();
    for (ei, edge) in edge_positions.iter().enumerate() {
        prev_pos = get_pos_from_line(&lines[0]);
        for i in 1..lines.len() {
            current_pos = get_pos_from_line(&lines[i]);
            if current_pos.0 == prev_pos.0 && edge.0 == current_pos.0 {
                if current_pos.1 > prev_pos.1 {
                    //dir = (0, 1);
                    if edge.1 > prev_pos.1 && edge.1 < current_pos.1 { bad_edges.push(ei); break; }
                } else {
                    //dir = (0, -1);
                    if edge.1 < prev_pos.1 && edge.1 > current_pos.1 { bad_edges.push(ei); break; }
                }
            } else if edge.1 == current_pos.1 {
                if current_pos.0 > prev_pos.0 {
                    //dir = (1, 0);
                    if edge.0 > prev_pos.0 && edge.0 < current_pos.0 { bad_edges.push(ei); break; }
                } else {
                    //dir = (-1, 0);
                    if edge.1 < prev_pos.1 && edge.1 > current_pos.1 { bad_edges.push(ei); break; }
                }
            }
        }
    }
    dbg!(edge_positions.len());
    dbg!(bad_edges.len());
    let mut true_edges_positions: Vec<(u64, u64)> = Vec::new();
    for (ei, edge) in edge_positions.iter().enumerate() {
        if !bad_edges.contains(&ei) {
            true_edges_positions.push(*edge);
        }
    }
    dbg!(true_edges_positions.len());
    println!("done with that");*/

    edge_positions
}

fn get_nof_right_and_left_turns(lines: &Vec<String>) -> (u64, u64) {
    let mut right_turns: u64 = 0;
    let mut left_turns: u64 = 0;

    let mut prev_pos: (u64, u64);
    let mut current_pos: (u64, u64);
    let mut prev_dir: (i32, i32);

    let mut iter = lines[0].split(',');
    prev_pos = (iter.next().unwrap().parse().unwrap(), iter.next().unwrap().parse().unwrap());
    let mut iter = lines[1].split(',');
    current_pos = (iter.next().unwrap().parse().unwrap(), iter.next().unwrap().parse().unwrap());

    if current_pos.0 == prev_pos.0 {
        if current_pos.1 > prev_pos.1 {
            prev_dir = (0, 1);
        } else {
            prev_dir = (0, -1);
        }
    } else {
        if current_pos.0 > prev_pos.0 {
            prev_dir = (1, 0);
        } else {
            prev_dir = (-1, 0);
        }
    }

    for i in 2..lines.len() {
        prev_pos = current_pos;
        let mut line_iter = lines[i].split(',');
        current_pos = (line_iter.next().unwrap().parse().unwrap(), line_iter.next().unwrap().parse().unwrap());

        match prev_dir {
            (1, 0) => {
                if current_pos.0 == prev_pos.0 {
                    if current_pos.1 > prev_pos.1 {
                        right_turns += 1;
                        prev_dir = (0, 1);
                    } else {
                        left_turns += 1;
                        prev_dir = (0, -1);
                    }
                } else {
                    if current_pos.0 > prev_pos.0 {
                        // no turn
                        prev_dir = (1, 0);
                    } else {
                        // no turn
                        prev_dir = (-1, 0);
                    }
                }
            },
            (-1, 0) => {
                if current_pos.0 == prev_pos.0 {
                    if current_pos.1 > prev_pos.1 {
                        left_turns += 1;
                        prev_dir = (0, 1);
                    } else {
                        right_turns += 1;
                        prev_dir = (0, -1);
                    }
                } else {
                    if current_pos.0 > prev_pos.0 {
                        // no turn
                        prev_dir = (1, 0);
                    } else {
                        // no turn
                        prev_dir = (-1, 0);
                    }
                }
            },
            (0, 1) => {
                if current_pos.0 == prev_pos.0 {
                    if current_pos.1 > prev_pos.1 {
                        // no turn
                        prev_dir = (0, 1);
                    } else {
                        // no turn
                        prev_dir = (0, -1);
                    }
                } else {
                    if current_pos.0 > prev_pos.0 {
                        left_turns += 1;
                        prev_dir = (1, 0);
                    } else {
                        right_turns += 1;
                        prev_dir = (-1, 0);
                    }
                }
            },
            (0, -1) => {
                if current_pos.0 == prev_pos.0 {
                    if current_pos.1 > prev_pos.1 {
                        // no turn
                        prev_dir = (0, 1);
                    } else {
                        // no turn
                        prev_dir = (0, -1);
                    }
                } else {
                    if current_pos.0 > prev_pos.0 {
                        right_turns += 1;
                        prev_dir = (1, 0);
                    } else {
                        left_turns += 1;
                        prev_dir = (-1, 0);
                    }
                }
            },
            _ => {assert!(false);},
        }
    }
    (right_turns, left_turns)
}

fn get_bad_positions_for_redtiles(lines: &Vec<String>, right: bool, edges: &Vec<(u64, u64)>) -> Vec<Vec<bool>> {

    let mut bad_pos: Vec<Vec<bool>> = vec![vec![false; 8]; lines.len()];

    for i in 0..lines.len()-1 {
        let current_pos: (u64, u64) = get_pos_from_line(&lines[i]);
        let next_pos: (u64, u64) = get_pos_from_line(&lines[i+1]);

        if next_pos.0 == current_pos.0 {
            if next_pos.1 > current_pos.1 {
                //prev_dir = (0, 1);
                bad_pos[i][2] = false;
                bad_pos[i+1][0] = false;
                if right { bad_pos[i][3] = false; bad_pos[i][6] = false; bad_pos[i][7] = false; bad_pos[i+1][3] = false; bad_pos[i+1][6] = false; bad_pos[i+1][7] = false; }
                else { bad_pos[i][4] = false; bad_pos[i][1] = false; bad_pos[i][5] = false; bad_pos[i+1][4] = false; bad_pos[i+1][1] = false; bad_pos[i+1][5] = false; }
            } else {
                //prev_dir = (0, -1);
                bad_pos[i][0] = false;
                bad_pos[i+1][2] = false;
                if right { bad_pos[i][4] = false; bad_pos[i][1] = false; bad_pos[i][5] = false; bad_pos[i+1][4] = false; bad_pos[i+1][1] = false; bad_pos[i+1][5] = false; }
                else { bad_pos[i][3] = false; bad_pos[i][6] = false; bad_pos[i][7] = false; bad_pos[i+1][3] = false; bad_pos[i+1][6] = false; bad_pos[i+1][7] = false; }
            }
        } else {
            if next_pos.0 > current_pos.0 {
                //prev_dir = (1, 0);
                bad_pos[i][1] = false;
                bad_pos[i+1][3] = false;
                if right { bad_pos[i][6] = false; bad_pos[i][2] = false; bad_pos[i][5] = false; bad_pos[i+1][6] = false; bad_pos[i+1][2] = false; bad_pos[i+1][5] = false; }
                else { bad_pos[i][7] = false; bad_pos[i][0] = false; bad_pos[i][4] = false; bad_pos[i+1][7] = false; bad_pos[i+1][0] = false; bad_pos[i+1][4] = false; }
            } else {
                //prev_dir = (-1, 0);
                bad_pos[i][3] = false;
                bad_pos[i+1][1] = false;
                if right { bad_pos[i][7] = false; bad_pos[i][0] = false; bad_pos[i][4] = false; bad_pos[i+1][7] = false; bad_pos[i+1][0] = false; bad_pos[i+1][4] = false; }
                else { bad_pos[i][6] = false; bad_pos[i][2] = false; bad_pos[i][5] = false; bad_pos[i+1][6] = false; bad_pos[i+1][2] = false; bad_pos[i+1][5] = false; }
            }
        }
    }
    
    println!("starting edges*lines compute");
    for i in 0..lines.len()-1 {
        let p: (u64, u64) = get_pos_from_line(&lines[i]);
        if bad_pos[i][0] { if edges.contains(&(p.0, p.1-1)) { bad_pos[i][0] = false; } }
        if bad_pos[i][1] { if edges.contains(&(p.0+1, p.1)) { bad_pos[i][1] = false; } }
        if bad_pos[i][2] { if edges.contains(&(p.0, p.1+1)) { bad_pos[i][2] = false; } }
        if bad_pos[i][3] { if edges.contains(&(p.0-1, p.1)) { bad_pos[i][3] = false; } }
        
        if bad_pos[i][4] { if edges.contains(&(p.0+1, p.1-1)) { bad_pos[i][4] = false; } }
        if bad_pos[i][5] { if edges.contains(&(p.0+1, p.1+1)) { bad_pos[i][5] = false; } }
        if bad_pos[i][6] { if edges.contains(&(p.0-1, p.1+1)) { bad_pos[i][6] = false; } }
        if bad_pos[i][7] { if edges.contains(&(p.0-1, p.1-1)) { bad_pos[i][7] = false; } }
    }
    println!("done with that");

    bad_pos
}



pub fn solve_p2() -> u64 {
    let mut lines: Vec<String> = utils::read_filelines_to_vec(INPUT_PATH);
    lines.push(lines[0].clone());

    let red_tiles: Vec<(u64, u64)> = lines.iter().map(|s: &String| get_pos_from_line(s)).collect();
    let mut biggest_area: u64 = 0;

    

    let (rights, lefts) = get_nof_right_and_left_turns(&lines);
    dbg!(rights, lefts);

    let edges: Vec<(u64, u64)> = get_edge_positions(&lines);
    let off_edges: Vec<(u64, u64)> = get_offedge_positions(&lines, rights > lefts);

    
    let bad_positions: Vec<Vec<bool>> = get_bad_positions_for_redtiles(&lines, rights > lefts, &edges);

    /*
    dbg!(edges.len());
    edges.sort_by(|a, b| a.0.cmp(&b.0));
    println!("first srot done");
    edges.sort_by(|a, b| a.1.cmp(&b.1));
    
    let mut prev = edges[0];
    for i in 1..edges.len() {
        let current = edges[i];
        if current == prev {
            println!("CHECK");
        }
        prev = current;
    }
    
    for red_tile in &red_tiles {
        if edges.contains(red_tile) {
            println!("EDGES CONTAIN REDTILE");
        }

        if red_tiles.contains(&(red_tile.0+1, red_tile.1)) { println!("REDILTES HAS NEIGHBR REDTILE"); }
        if red_tiles.contains(&(red_tile.0-1, red_tile.1)) { println!("REDILTES HAS NEIGHBR REDTILE"); }
        if red_tiles.contains(&(red_tile.0, red_tile.1+1)) { println!("REDILTES HAS NEIGHBR REDTILE"); }
        if red_tiles.contains(&(red_tile.0, red_tile.1-1)) { println!("REDILTES HAS NEIGHBR REDTILE"); }

        if red_tiles.contains(&(red_tile.0+1, red_tile.1+1)) { println!("REDILTES HAS NEIGHBR REDTILE diag"); }
        if red_tiles.contains(&(red_tile.0-1, red_tile.1-1)) { println!("REDILTES HAS NEIGHBR REDTILE diag"); }
        if red_tiles.contains(&(red_tile.0-1, red_tile.1+1)) { println!("REDILTES HAS NEIGHBR REDTILE diag"); }
        if red_tiles.contains(&(red_tile.0+1, red_tile.1-1)) { println!("REDILTES HAS NEIGHBR REDTILE diag"); }

        if edges.contains(&(red_tile.0+1, red_tile.1)) { println!("edges HAS NEIGHBR REDTILE"); }
        if edges.contains(&(red_tile.0-1, red_tile.1)) { println!("edges HAS NEIGHBR REDTILE"); }
        if edges.contains(&(red_tile.0, red_tile.1+1)) { println!("edges HAS NEIGHBR REDTILE"); }
        if edges.contains(&(red_tile.0, red_tile.1-1)) { println!("edges HAS NEIGHBR REDTILE"); }
    }*/


    println!("starting final area computing");
    for i in 0..red_tiles.len()-1 {
        let p1: (u64, u64) = red_tiles[i];
        for j in (i+1)..red_tiles.len() {
            let p2: (u64, u64) = red_tiles[j];
            let area: u64 = get_area_from_corners((p1.0 as i64, p1.1 as i64), (p2.0 as i64, p2.1 as i64));

            if area > biggest_area {
                if area_between_corners_is_valid(p1, p2, &red_tiles, &bad_positions, &off_edges) {
                    biggest_area = area;
                }
            }
        }
    }

    biggest_area
}

fn area_between_corners_is_valid(p1: (u64, u64), p2: (u64, u64), red_tiles: &Vec<(u64, u64)>, bad_positions: &Vec<Vec<bool>>, off_edges: &Vec<(u64, u64)>) -> bool {
    if p1.0 == p2.0 {
        if p1.1 > p2.1 {
            for (ri, (rx, ry)) in red_tiles.iter().enumerate() {
                if *rx == p1.0 && *ry > p2.1 && *ry < p1.1 {
                    if (bad_positions[ri][0] && *ry != (p2.1+1)) || 
                    (bad_positions[ri][2] && *ry != (p1.1-1)) {
                        return false;
                    }
                }
                if (*rx, *ry) == p1 && bad_positions[ri][0] { return false; }
                if (*rx, *ry) == p2 && bad_positions[ri][2] { return false; }
            }

            for off_edge in off_edges {
                if off_edge.0 == p1.0 && off_edge.1 > p2.1 && off_edge.1 < p1.1 { return false; }
            }
        } else {
            for (ri, (rx, ry)) in red_tiles.iter().enumerate() {
                if *rx == p1.0 && *ry < p2.1 && *ry > p1.1 {
                    if (bad_positions[ri][0] && *ry != (p2.1-1)) || 
                    (bad_positions[ri][2] && *ry != (p1.1+1)) {
                        return false;
                    }
                }
                if (*rx, *ry) == p1 && bad_positions[ri][2] { return false; }
                if (*rx, *ry) == p2 && bad_positions[ri][0] { return false; }
            }

            for off_edge in off_edges {
                if off_edge.0 == p1.0 && off_edge.1 > p1.1 && off_edge.1 < p2.1 { return false; }
            }
        }
    } else if p1.1 == p2.1 {
        if p1.0 > p2.0 {
            for (ri, (rx, ry)) in red_tiles.iter().enumerate() {
                if *ry == p1.1 && *rx > p2.0 && *rx < p1.0 {
                    if (bad_positions[ri][3] && *rx != (p2.0+1)) || 
                    (bad_positions[ri][1] && *rx != (p1.0-1)) {
                        return false;
                    }
                }
                if (*rx, *ry) == p1 && bad_positions[ri][3] { return false; }
                if (*rx, *ry) == p2 && bad_positions[ri][1] { return false; }
            }

            for off_edge in off_edges {
                if off_edge.1 == p1.1 && off_edge.0 > p2.0 && off_edge.0 < p1.0 { return false; }
            }

        } else {
            for (ri, (rx, ry)) in red_tiles.iter().enumerate() {
                if *ry == p1.1 && *rx < p2.0 && *rx > p1.0 {
                    if (bad_positions[ri][3] && *rx != (p2.0-1)) || 
                    (bad_positions[ri][1] && *rx != (p1.0+1)) {
                        return false;
                    }
                }
                if (*rx, *ry) == p1 && bad_positions[ri][1] { return false; }
                if (*rx, *ry) == p2 && bad_positions[ri][3] { return false; }
            }

            for off_edge in off_edges {
                if off_edge.1 == p1.1 && off_edge.0 > p1.0 && off_edge.0 < p2.0 { return false; }
            }
        }
    } else {
        // p1.1 != p2.1 && p1.0 != p2.0
        let smaller_larger_x: (u64, u64);
        let smaller_larger_y: (u64, u64);
        if p1.1 > p2.1 {
            smaller_larger_y = (p2.1, p1.1);
        } else { smaller_larger_y = (p1.1, p2.1); }
        if p1.0 > p2.0 {
            smaller_larger_x = (p2.0, p1.0);
        } else { smaller_larger_x = (p1.0, p2.0); }

        for (ri, (rx, ry)) in red_tiles.iter().enumerate() {

            if (*rx, *ry) == p1 || (*rx, *ry) == p2 {
                if p1.0 > p2.0 && p1.1 > p2.1 {
                    if (*rx, *ry) == p1 && (bad_positions[ri][0] || bad_positions[ri][3] || bad_positions[ri][7]) { return false; }
                    if (*rx, *ry) == p2 && (bad_positions[ri][2] || bad_positions[ri][1] || bad_positions[ri][5]) { return false; }
                } else if p2.0 > p1.0 && p1.1 > p2.1 {
                    if (*rx, *ry) == p1 && (bad_positions[ri][0] || bad_positions[ri][1] || bad_positions[ri][4]) { return false; }
                    if (*rx, *ry) == p2 && (bad_positions[ri][2] || bad_positions[ri][3] || bad_positions[ri][6]) { return false; }
                } else if p1.0 < p2.0 && p1.1 < p2.1 {
                    if (*rx, *ry) == p1 && (bad_positions[ri][1] || bad_positions[ri][2] || bad_positions[ri][5]) { return false; }
                    if (*rx, *ry) == p2 && (bad_positions[ri][0] || bad_positions[ri][3] || bad_positions[ri][7]) { return false; }
                } else if p1.0 > p2.0 && p1.1 < p2.1 {
                    if (*rx, *ry) == p1 && (bad_positions[ri][2] || bad_positions[ri][3] || bad_positions[ri][6]) { return false; }
                    if (*rx, *ry) == p2 && (bad_positions[ri][1] || bad_positions[ri][0] || bad_positions[ri][4]) { return false; }
                }

            } else if *rx > smaller_larger_x.0 && *rx < smaller_larger_x.1 &&
            *ry > smaller_larger_y.0 && *ry < smaller_larger_y.1 {
                if bad_positions[ri].contains(&true) {
                    return false;
                }
                //println!("check");
                //return false;
            } else if *rx == smaller_larger_x.0 {
                if *ry == smaller_larger_y.0 || *ry == smaller_larger_y.1 {
                    if *ry == smaller_larger_y.0 && (bad_positions[ri][1] || bad_positions[ri][2] || bad_positions[ri][5]) { return false; }
                    else if *ry == smaller_larger_y.1 && (bad_positions[ri][0] || bad_positions[ri][4] || bad_positions[ri][1]) { return false; }
                }
                else if bad_positions[ri][0] || bad_positions[ri][1] || bad_positions[ri][2] || bad_positions[ri][4] || bad_positions[ri][5] {
                    return false;
                }
            } else if *rx == smaller_larger_x.1 {
                if *ry == smaller_larger_y.0 || *ry == smaller_larger_y.1 {
                    if *ry == smaller_larger_y.0 && (bad_positions[ri][3] || bad_positions[ri][2] || bad_positions[ri][6]) { return false; }
                    else if *ry == smaller_larger_y.1 && (bad_positions[ri][0] || bad_positions[ri][7] || bad_positions[ri][3]) { return false; }
                }
                else if bad_positions[ri][0] || bad_positions[ri][3] || bad_positions[ri][2] || bad_positions[ri][7] || bad_positions[ri][6] {
                    return false;
                }
            } else if *ry == smaller_larger_y.0 {
                if *rx == smaller_larger_x.0 || *rx == smaller_larger_x.1 {
                    if *rx == smaller_larger_x.0 && (bad_positions[ri][1] || bad_positions[ri][2] || bad_positions[ri][5]) { return false; }
                    else if *rx == smaller_larger_x.1 && (bad_positions[ri][3] || bad_positions[ri][6] || bad_positions[ri][2]) { return false; }
                }
                else if bad_positions[ri][3] || bad_positions[ri][2] || bad_positions[ri][1] || bad_positions[ri][6] || bad_positions[ri][5] {
                    return false;
                }
            } else if *ry == smaller_larger_y.1 {
                if *rx == smaller_larger_x.0 || *rx == smaller_larger_x.1 {
                    if *rx == smaller_larger_x.0 && (bad_positions[ri][1] || bad_positions[ri][0] || bad_positions[ri][4]) { return false; }
                    else if *rx == smaller_larger_x.1 && (bad_positions[ri][0] || bad_positions[ri][3] || bad_positions[ri][7]) { return false; }
                }
                else if bad_positions[ri][3] || bad_positions[ri][0] || bad_positions[ri][1] || bad_positions[ri][4] || bad_positions[ri][7] {
                    return false;
                }
            }
        }

        for off_edge in off_edges {
            if off_edge.0 >= smaller_larger_x.0 && off_edge.0 <= smaller_larger_x.1 &&
            off_edge.1 >= smaller_larger_y.0 && off_edge.1 <= smaller_larger_y.1 {
                return false;
            }
        }
    }

    true
}

