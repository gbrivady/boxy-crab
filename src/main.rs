use core::fmt;
use std::cmp;
use std::ops;

struct Grid(Vec<Vec<bool>>);

impl ops::Deref for Grid {
    type Target = Vec<Vec<bool>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let move_back_x = 2 * self[0].len();
        self.iter().fold(Ok(()), |result, line| {
            result.and_then(|_| {
                line.iter()
                    .fold(Ok(()), |result_inner, cell| {
                        result_inner.and_then(|_| {
                            if *cell {
                                write!(f, "\u{2588}\u{2588}")
                            } else {
                                write!(f, "  ")
                            }
                        })
                    })
                    .and_then(|_| write!(f, "\u{001b}[{move_back_x}D\u{001b}[1B"))
            })
        })
    }
}

const GRID_A: [[bool; 5]; 5] = [
    [false, true, true, true, false],
    [false, true, false, true, false],
    [false, true, true, true, false],
    [false, true, false, true, false],
    [false, true, false, true, false],
];

const GRID_X: [[bool; 5]; 5] = [
    [true, false, false, false, true],
    [false, true, false, true, false],
    [false, false, true, false, false],
    [false, true, false, true, false],
    [true, false, false, false, true],
];

const GRID_DBG: [[bool; 5]; 5] = [
    [true, true, true, true, true],
    [true, false, true, false, true],
    [true, true, false, false, true],
    [true, true, false, true, false],
    [true, true, true, true, true],
];

macro_rules! const_grid_to_vec {
    ( $x:expr ) => {
        Grid($x.iter().map(|x| x.to_vec()).collect::<Vec<_>>())
    };
}

fn build_hints(g: &Grid) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut row_hints: Vec<Vec<i32>> = Vec::new();
    let mut col_hints: Vec<Vec<i32>> = vec![Vec::new(); g[0].len()];
    let mut col_counts: Vec<i32> = vec![0; g[0].len()];
    for row in g.iter() {
        let mut cur_hint: Vec<i32> = Vec::new();
        let mut cur_count: i32 = 0;
        for cell in row.iter().enumerate() {
            match (cell, cur_count) {
                ((j, &true), _) => {
                    cur_count += 1;
                    col_counts[j] += 1;
                }
                ((j, &false), 0) => {
                    if col_counts[j] != 0 {
                        col_hints[j].push(col_counts[j]);
                        col_counts[j] = 0;
                    }
                }
                ((j, &false), _) => {
                    cur_hint.push(cur_count);
                    cur_count = 0;
                    if col_counts[j] != 0 {
                        col_hints[j].push(col_counts[j]);
                        col_counts[j] = 0;
                    }
                }
            }
        }
        if cur_count != 0 {
            cur_hint.push(cur_count)
        }
        row_hints.push(cur_hint);
    }
    for (j, count) in col_counts.iter().enumerate() {
        if *count != 0 {
            col_hints[j].push(*count);
        }
    }
    return (row_hints, col_hints);
}

//unicode escape : \u{001b}
fn draw_hints(h_hints: Vec<Vec<i32>>, v_hints: Vec<Vec<i32>>) {
    // horizontal hints takes 3 spaces in width, vertical ones 2 in height
    let width_hints = 3 * &h_hints.iter().fold(0, |acc, vec| cmp::max(acc, vec.len())) + 2;
    let heigth_hints = 2 * &v_hints.iter().fold(0, |acc, vec| cmp::max(acc, vec.len())) + 2;
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
    print!("\u{001b}[{};{}f", heigth_hints + 2, width_hints + 2);
}

fn main() {
    // let (a, b): (Vec<Vec<i32>>, Vec<Vec<i32>>) = build_hints(const_grid_to_vec!(GRID_A));
    // println!("{a:#?}\n{b:#?}");
    let grid_x: Grid = const_grid_to_vec!(GRID_DBG);
    let (h_hints, v_hints) = build_hints(&grid_x);
    print!("\u{001b}[2J");
    draw_hints(h_hints, v_hints);
    // let grid_x: Grid = const_grid_to_vec!(GRID_X);
    // println!("{grid_x}");
}
