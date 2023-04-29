mod grid;
use grid::Grid;

mod tui;

fn main() {
    // let (a, b): (Vec<Vec<i32>>, Vec<Vec<i32>>) = build_hints(const_grid_to_vec!(GRID_A));
    // println!("{a:#?}\n{b:#?}");
    let grid_x: Grid = const_grid_to_vec!(grid::GRID_DBG);
    let (h_hints, v_hints) = grid::build_hints(&grid_x);
    print!("\u{001b}[2J");
    let (x, y) = tui::draw_hints(h_hints, v_hints);
    print!("\u{001b}[{};{}f", x + 2, y + 2);
    print!("{grid_x}");
    print!("\u{001b}[2B");
}
