use core::time;
use std::{
    io::{self, Read, Write},
    thread,
};

mod grid;
use grid::Grid;

mod tui;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn read_input() -> () {
    let mut buffer: [u8; 1] = [0u8; 1];
    let mut handle: std::io::Stdin = io::stdin();
    loop {
        handle
            .read_exact(&mut buffer)
            .expect("Failed to read user input");
        match buffer[0] as char {
            'w' => println!("hi"),
            'q' => {
                println!("Quitting...");
                break;
            }
            _ => println!("not hi"),
        }
    }
}

fn main() {
    print!("\u{001b}[?1049h"); //Go to secondary screen

    let grid_x: Grid = const_grid_to_vec!(Grid::DBG);
    let (h_hints, v_hints) = grid_x.build_hints();
    print!("\u{001b}[2J");
    let (x, y) = tui::draw_hints(h_hints, v_hints);
    print!("\u{001b}[{};{}f", x + 2, y + 2);
    print!("{grid_x}");
    print!("\u{001b}[2B");
    print!("\u{001b}[97;65p");

    io::stdout().flush().expect("Unable to flush stdout");
    enable_raw_mode().expect("Unable to enable raw mode");
    read_input();
    disable_raw_mode().expect("Unable to disable raw mode");
    io::stdout().flush().expect("Unable to flush stdout");

    thread::sleep(time::Duration::from_secs(2));
    print!("\u{001b}[?1049l"); //Back to main screen
}
