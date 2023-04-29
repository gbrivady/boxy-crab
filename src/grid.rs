use core::fmt;
use std::ops;

pub struct Grid(pub Vec<Vec<bool>>);

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

pub fn build_hints(g: &Grid) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
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

#[macro_export]
macro_rules! const_grid_to_vec {
    ( $x:expr ) => {
        Grid($x.iter().map(|x| x.to_vec()).collect::<Vec<_>>())
    };
}

pub const GRID_A: [[bool; 5]; 5] = [
    [false, true, true, true, false],
    [false, true, false, true, false],
    [false, true, true, true, false],
    [false, true, false, true, false],
    [false, true, false, true, false],
];

pub const GRID_X: [[bool; 5]; 5] = [
    [true, false, false, false, true],
    [false, true, false, true, false],
    [false, false, true, false, false],
    [false, true, false, true, false],
    [true, false, false, false, true],
];

pub const GRID_DBG: [[bool; 5]; 5] = [
    [true, true, true, true, true],
    [true, false, true, false, false],
    [true, true, false, false, true],
    [true, true, false, true, false],
    [true, true, true, true, true],
];
