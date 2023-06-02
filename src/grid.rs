use core::fmt;
use std::cmp;
use std::ops;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Cell {
    FULL = 1,
    EMPTY = 0,
    CROSS = 2,
    DOT = 3,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Cell::FULL => write!(f, "\u{2588}\u{2588}"),
            Cell::EMPTY => write!(f, "  "),
            Cell::CROSS => write!(f, "\u{2573} "),
            Cell::DOT => write!(f, "\u{25AA} "),
        }
    }
}

pub struct Grid(pub Vec<Vec<Cell>>);

impl ops::Deref for Grid {
    type Target = Vec<Vec<Cell>>;

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
                        result_inner.and_then(|_| write!(f, "{cell}"))
                    })
                    .and_then(|_| write!(f, "\u{001b}[{move_back_x}D\u{001b}[1B"))
            })
        })
    }
}

impl cmp::PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        self.iter().enumerate().fold(true, |acc, (i, line)| {
            acc && line.iter().enumerate().fold(true, |acc, (j, cell)| {
                acc && match (*cell, other[i][j]) {
                    (Cell::FULL, Cell::FULL) => true,
                    (Cell::FULL, _) => false,
                    (_, Cell::FULL) => false,
                    (_, _) => true,
                }
            })
        })
    }
}
impl cmp::Eq for Grid {}

impl Grid {
    pub fn build_hints(&self) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
        let mut row_hints: Vec<Vec<i32>> = Vec::new();
        let mut col_hints: Vec<Vec<i32>> = vec![Vec::new(); self[0].len()];
        let mut col_counts: Vec<i32> = vec![0; self[0].len()];
        for row in self.iter() {
            let mut cur_hint: Vec<i32> = Vec::new();
            let mut cur_count: i32 = 0;
            for cell in row.iter().enumerate() {
                match (cell, cur_count) {
                    ((j, Cell::EMPTY), _) => {
                        cur_count += 1;
                        col_counts[j] += 1;
                    }
                    ((j, _), 0) => {
                        if col_counts[j] != 0 {
                            col_hints[j].push(col_counts[j]);
                            col_counts[j] = 0;
                        }
                    }
                    ((j, _), _) => {
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

    pub const A: [[Cell; 5]; 5] = [
        [Cell::EMPTY, Cell::FULL, Cell::FULL, Cell::FULL, Cell::EMPTY],
        [
            Cell::EMPTY,
            Cell::FULL,
            Cell::EMPTY,
            Cell::FULL,
            Cell::EMPTY,
        ],
        [Cell::EMPTY, Cell::FULL, Cell::FULL, Cell::FULL, Cell::EMPTY],
        [
            Cell::EMPTY,
            Cell::FULL,
            Cell::EMPTY,
            Cell::FULL,
            Cell::EMPTY,
        ],
        [
            Cell::EMPTY,
            Cell::FULL,
            Cell::EMPTY,
            Cell::FULL,
            Cell::EMPTY,
        ],
    ];

    pub const X: [[Cell; 5]; 5] = [
        [
            Cell::FULL,
            Cell::EMPTY,
            Cell::EMPTY,
            Cell::EMPTY,
            Cell::FULL,
        ],
        [
            Cell::EMPTY,
            Cell::FULL,
            Cell::EMPTY,
            Cell::FULL,
            Cell::EMPTY,
        ],
        [
            Cell::EMPTY,
            Cell::EMPTY,
            Cell::FULL,
            Cell::EMPTY,
            Cell::EMPTY,
        ],
        [
            Cell::EMPTY,
            Cell::FULL,
            Cell::EMPTY,
            Cell::FULL,
            Cell::EMPTY,
        ],
        [
            Cell::FULL,
            Cell::EMPTY,
            Cell::EMPTY,
            Cell::EMPTY,
            Cell::FULL,
        ],
    ];

    pub const DBG: [[Cell; 5]; 5] = [
        [Cell::FULL, Cell::FULL, Cell::FULL, Cell::FULL, Cell::FULL],
        [
            Cell::FULL,
            Cell::EMPTY,
            Cell::FULL,
            Cell::EMPTY,
            Cell::EMPTY,
        ],
        [Cell::FULL, Cell::FULL, Cell::EMPTY, Cell::EMPTY, Cell::FULL],
        [Cell::FULL, Cell::FULL, Cell::EMPTY, Cell::FULL, Cell::EMPTY],
        [Cell::FULL, Cell::FULL, Cell::FULL, Cell::FULL, Cell::FULL],
    ];
}

#[macro_export]
macro_rules! const_grid_to_vec {
    ( $x:expr ) => {
        Grid($x.iter().map(|x| x.to_vec()).collect::<Vec<_>>())
    };
}
