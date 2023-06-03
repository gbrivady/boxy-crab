use core::time;
use std::{
    io::{self, Read, Write},
    thread,
};

mod grid;
use grid::Grid;

mod tdisplay;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
pub enum Move {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE,
}

fn legal_move(input: Move, cursor: (usize, usize), grid_dim: (usize, usize)) -> bool {
    match input {
        Move::UP => cursor.0 > 0,
        Move::DOWN => cursor.0 + 1 < grid_dim.0,
        Move::LEFT => cursor.1 > 0,
        Move::RIGHT => cursor.1 + 1 < grid_dim.1,
        _ => true,
    }
}

fn main() {
    print!("\u{001b}[?1049h"); //Go to secondary screen

    print!("\u{001b}[2J"); //Clear screen

    // Build hints and two grid : puzzle grid and user grid
    let grid: Grid = const_grid_to_vec!(Grid::DBG);
    let (h_hints, v_hints) = grid.build_hints();
    let grid_dim: (usize, usize) = (
        grid.len().try_into().unwrap(),
        grid[0].len().try_into().unwrap(),
    );
    let mut user_grid: Grid = Grid::make_empty(grid_dim.0, grid_dim.1);

    // Display hints and set cursor to the right position
    let (x, y) = tdisplay::draw_hints(h_hints, v_hints);
    print!("\u{001b}[{};{}f", x + 2, y + 2);
    print!("{user_grid}");
    print!("\u{001b}[{}A", grid_dim.0);
    io::stdout().flush().expect("Unable to flush stdout");

    // Setup game variables
    // 1 - IO
    let mut buffer: [u8; 1] = [0u8; 1];
    let mut handle: std::io::Stdin = io::stdin();
    let mut input: Move;
    let output_line: u32 = y + 4 + grid_dim.0 as u32;

    // 2 - Game
    let mut cursor: (usize, usize) = (0, 0);
    let mut old_cell: grid::Cell;
    let mut new_cell: grid::Cell = user_grid[cursor.0][cursor.1];

    // Set raw mode before main loop
    enable_raw_mode().expect("Unable to enable raw mode");

    // Game main loop
    loop {
        old_cell = new_cell;
        handle
            .read_exact(&mut buffer)
            .expect("Failed to read user input");
        input = match buffer[0] as char {
            'q' => {
                tdisplay::output_message("\u{001b}[;39;39mQuitting...", output_line);
                break;
            }
            'w' => {
                if legal_move(Move::UP, cursor, grid_dim) {
                    cursor.0 -= 1;
                    Move::UP
                } else {
                    Move::NONE
                }
            }
            's' => {
                if legal_move(Move::DOWN, cursor, grid_dim) {
                    cursor.0 += 1;
                    Move::DOWN
                } else {
                    Move::NONE
                }
            }
            'a' => {
                if legal_move(Move::LEFT, cursor, grid_dim) {
                    cursor.1 -= 1;
                    Move::LEFT
                } else {
                    Move::NONE
                }
            }
            'd' => {
                if legal_move(Move::RIGHT, cursor, grid_dim) {
                    cursor.1 += 1;
                    Move::RIGHT
                } else {
                    Move::NONE
                }
            }
            'x' => {
                user_grid[cursor.0][cursor.1].switch_to(grid::Cell::CROSS);
                Move::NONE
            }
            'e' => {
                user_grid[cursor.0][cursor.1].switch_to(grid::Cell::FULL);
                Move::NONE
            }
            'r' => {
                user_grid[cursor.0][cursor.1].switch_to(grid::Cell::EMPTY);
                Move::NONE
            }
            'f' => {
                user_grid[cursor.0][cursor.1].switch_to(grid::Cell::DOT);
                Move::NONE
            }
            _ => continue,
        };
        new_cell = user_grid[cursor.0][cursor.1];
        tdisplay::update_cursor(input, old_cell, new_cell);
        io::stdout().flush().expect("Unable to flush stdout");
    }

    // Exit raw mode, flush any leftover input
    disable_raw_mode().expect("Unable to disable raw mode");
    io::stdout().flush().expect("Unable to flush stdout");

    // Slight pause after to display quitting text
    thread::sleep(time::Duration::from_secs(1));
    println!("\u{001b}[?25h\u{001b}[?1049l"); //Back to main screen, de-hides cursor
}
