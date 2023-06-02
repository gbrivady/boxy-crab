use core::time;
use std::{
    io::{self, stdout, Read, Write},
    thread,
};

mod grid;
use grid::Grid;

mod tdisplay;

use crossterm::{
    cursor,
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};

fn main() {
    print!("\u{001b}[?1049h"); //Go to secondary screen
    stdout()
        .execute(cursor::Hide)
        .expect("Unable to hide cursor");
    let grid_x: Grid = const_grid_to_vec!(Grid::DBG);
    let (h_hints, v_hints) = grid_x.build_hints();
    print!("\u{001b}[2J");
    let (x, y) = tdisplay::draw_hints(h_hints, v_hints);
    print!("\u{001b}[{};{}f", x + 2, y + 2);
    print!("{grid_x}");
    print!("\u{001b}[2B");
    print!("\u{001b}[97;65p");

    io::stdout().flush().expect("Unable to flush stdout");
    enable_raw_mode().expect("Unable to enable raw mode");
    tdisplay::read_input();
    disable_raw_mode().expect("Unable to disable raw mode");
    io::stdout().flush().expect("Unable to flush stdout");

    thread::sleep(time::Duration::from_secs(1));
    print!("\u{001b}[?1049l"); //Back to main screen
}
