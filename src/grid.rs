use rand::Rng;
use std::fmt;

mod cell {
    #[derive(Clone, PartialEq)]
    pub enum Status {
        Nothing,
        Mine,
        NearMine(u8),
    }

    #[derive(Clone)]
    pub struct Cell {
        pub status: Status,
        pub discovered: bool,
    }

    impl Cell {
        pub fn new() -> Self {
            Cell {
                status: Status::Nothing,
                discovered: false,
            }
        }
    }
}

pub struct Grid {
    rows: Vec<Vec<cell::Cell>>,
    dimensions: (usize, usize),
}

static STANDARD_DIMENSIONS: (usize, usize) = (10usize, 10usize);

impl Grid {
    fn make_empty() -> Self {
        Grid {
            rows: vec![vec![cell::Cell::new(); STANDARD_DIMENSIONS.1]; STANDARD_DIMENSIONS.0],
            dimensions: STANDARD_DIMENSIONS,
        }
    }

    fn generate_bombs(&mut self) {
        static STANDARD_MINE_RATE: i32 = 5i32;

        for cell in self.rows.iter_mut().flat_map(|row| row.iter_mut()) {
            let rand = rand::thread_rng().gen_range(1..=100);

            if rand <= STANDARD_MINE_RATE {
                cell.status = cell::Status::Mine;
            }
        }
    }

    fn count_near_mines(&self, position: (usize, usize)) -> u8 {
        static NEAR_CELL_MODIFIERS: [(isize, isize); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (0, 1),
            (-1, 1),
            (1, 0),
            (1, 1),
        ];
        let mut nb_mines = 0u8;

        for modifier in NEAR_CELL_MODIFIERS {
            let checking_position: (isize, isize) = (
                (position.0 as isize) + modifier.0,
                (position.1 as isize) + modifier.1,
            );

            if checking_position.0 >= 0
                && checking_position.0 < (self.dimensions.0 as isize)
                && checking_position.1 >= 0
                && checking_position.1 < (self.dimensions.1 as isize)
            {
                if self.rows[(checking_position.0 as usize)][(checking_position.1 as usize)].status
                    == cell::Status::Mine
                {
                    nb_mines += 1u8;
                }
            }
        }
        nb_mines
    }

    fn generate_hints(&mut self) {
        for x in 0..self.dimensions.0 {
            for y in 0..self.dimensions.1 {
                let nb_mines = self.count_near_mines((x, y));
                let cell: &mut cell::Cell = &mut self.rows[x][y];

                if nb_mines > 0u8 && cell.status != cell::Status::Mine {
                    cell.status = cell::Status::NearMine(nb_mines);
                }
            }
        }
    }

    pub fn make_random() -> Self {
        let mut grid = Grid::make_empty();

        grid.generate_bombs();
        grid.generate_hints();
        grid
    }

    pub fn to_debug_str(self) -> String {
        let mut str = String::new();

        for row in &self.rows {
            for cell in row {
                match cell.status {
                    cell::Status::Nothing => str.push_str(" "),
                    cell::Status::Mine => str.push_str("X"),
                    cell::Status::NearMine(nb_bombs) => match nb_bombs {
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
        for row in &self.rows {
            for cell in row {
                if cell.discovered {
                    match cell.status {
                        cell::Status::Nothing => write!(f, " ")?,
                        cell::Status::Mine => write!(f, "X")?,
                        cell::Status::NearMine(nb_bombs) => match nb_bombs {
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
