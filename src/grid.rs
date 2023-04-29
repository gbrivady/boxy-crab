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
