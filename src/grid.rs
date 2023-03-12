enum Status {
    Nothing,
    Mine,
    NearBomb(u8),
}

struct Cell {
    status: Status,
    discovered: bool,
}

pub struct Grid {
    grid: Vec<Vec<Cell>>,
    dimensions: (u8, u8),
}

use std::fmt;

impl Grid {
    fn new() -> Self {
    }
}

// impl From<String> for Grid {
//     fn from(s: String) -> Self {
//     }
// }

// impl fmt::Display for Grid {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     }
// }