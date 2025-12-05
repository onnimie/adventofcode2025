#![allow(dead_code)]

mod utils;
mod d1;
mod d2;
mod d3;
mod d4;
mod d5;

fn main() {
    println!("Hello, world!");


    let solution = d5::solve_p2();
    println!("Solution: {}", solution);
}
