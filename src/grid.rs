use std::fmt;

#[derive(Clone)]
pub enum Status {
    Nothing,
    Mine,
    NearBomb(u8),
}

#[derive(Clone)]
pub struct Cell {
    status: Status,
    discovered: bool,
}

pub struct Grid {
    grid: Vec<Vec<Cell>>,
    dimensions: (usize, usize),
}

static STANDARD_DIMENSIONS: (usize, usize) = (10usize, 10usize);

impl Grid {
    pub fn make_empty() -> Self {
        Grid {
            grid: vec![
                vec![
                    Cell {
                        status: Status::Nothing,
                        discovered: false
                    };
                    STANDARD_DIMENSIONS.1
                ];
                STANDARD_DIMENSIONS.0
            ],
            dimensions: STANDARD_DIMENSIONS,
        }
    }

    pub fn to_debug_str(self) -> String {
        let mut str = String::new();

        for row in &self.grid {
            for cell in row {
                match cell.status {
                    Status::Nothing => str.push_str(" "),
                    Status::Mine => str.push_str("X"),
                    Status::NearBomb(nb_bombs) => match nb_bombs {
                        1..=9 => str.push_str(&nb_bombs.to_string()),
                        _ => str.push_str("?"),
                    },
                };
            }
            str.push_str("\n");
        }
        str
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for cell in row {
                if cell.discovered {
                    match cell.status {
                        Status::Nothing => write!(f, " ")?,
                        Status::Mine => write!(f, "X")?,
                        Status::NearBomb(nb_bombs) => match nb_bombs {
                            1..=9 => write!(f, "{}", nb_bombs)?,
                            _ => write!(f, "?")?,
                        },
                    };
                } else {
                    write!(f, "#")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
