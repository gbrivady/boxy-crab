use std::cmp;

use crate::grid::Cell;
use crate::Move;

pub fn draw_hints(h_hints: Vec<Vec<i32>>, v_hints: Vec<Vec<i32>>) -> (u32, u32) {
    // horizontal hints takes 3 spaces in width, vertical ones 2 in height
    let width_hints = 3 * &h_hints.iter().fold(0, |acc, vec| cmp::max(acc, vec.len())) + 2;
    let heigth_hints = 2 * &v_hints.iter().fold(0, |acc, vec| cmp::max(acc, vec.len())) + 1;
    //move cursor to just over the grid
    print!("\u{001b}[{};{}f", heigth_hints, width_hints + 2);
    for vec in v_hints.iter() {
        for hint in vec.iter().rev() {
            //print char, move two lines up, 2 columns left
            print!("{hint:>2}\u{001b}[2A\u{001b}[2D");
        }
        //at end of column, go to next one
        print!("\u{001b}[{}B\u{001b}[2C", 2 * vec.len());
    }
    //move cursor to just left of the grid
    print!("\u{001b}[{};{}f", heigth_hints + 2, width_hints - 2);
    for vec in h_hints.iter() {
        for hint in vec.iter().rev() {
            print!("{hint:>2}\u{001b}[5D");
        }
        print!("\u{001b}[1B\u{001b}[{}G", width_hints - 2);
    }
    //reset cursor for grid drawing purposes
    return (
        heigth_hints.try_into().unwrap(),
        width_hints.try_into().unwrap(),
    );
}

pub fn update_cursor(input: Move, old: Cell, new: Cell) -> () {
    //reset old cell to normal
    print!("\u{001b}[;39;49m{old}\u{001b}[2D");

    // move cursor
    match input {
        Move::UP => print!("\u{001b}[1A"),
        Move::DOWN => print!("\u{001b}[1B"),
        Move::LEFT => print!("\u{001b}[2D"),
        Move::RIGHT => print!("\u{001b}[2C"),
        _ => (),
    }
    print!("\u{001b}[;91;41m{new}\u{001b}[2D");
    print!("\u{001b}[?25l");
}
