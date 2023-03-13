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
    pub fn new() -> Self {
        let mut rows: Vec<Vec<Cell>> = Vec::new();

        for _ in 0..STANDARD_DIMENSIONS.0 {
            rows.push(vec![
                Cell {
                    status: Status::Nothing,
                    discovered: false
                };
                STANDARD_DIMENSIONS.1
            ]);
        }

        Grid {
            grid: rows,
            dimensions: STANDARD_DIMENSIONS,
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, fd: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.grid.iter().map(|col| {
            col.iter().map(|cell| {
                if (cell.discovered) {

                } else {
                    // write!(fd, "#")?;
                }
            });
        });
        // for row in &self.grid {
        //     for cell in row {
        //         let status_print = match cell.discovered {
        //             false => "#",
        //             true => match cell.status {
        //                 Status::Nothing => " ",
        //                 Status::Mine => "!",
        //                 Status::NearBomb(nb_bombs) => match nb_bombs {
        //                     // 1..=9 => nb_bombs.to_string().as_str(),
        //                     _ => "?",
        //                 },
        //             },
        //         };
        //         write!(f, "{}", status_print)?;
        //     }
        //     write!(f, "\n")?;
        // }
        Ok(())
    }
}
