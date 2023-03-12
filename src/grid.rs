// enum Status {
//     Nothing,
//     Mine,
//     NearBomb(u8),
// }

// struct Cell {
//     status: Status,
//     discovered: bool,
// }

// type Grid = Vec<Vec<Cell>>;

use std::fmt;

pub struct Grid {
    rows: Vec<Vec<char>>,
}

impl From<String> for Grid {
    fn from(s: String) -> Self {
        let mut rows = Vec::new();
        for line in s.lines() {
            let row = line.chars().collect();
            rows.push(row);
        }
        Grid { rows }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.rows {
            for c in row {
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}